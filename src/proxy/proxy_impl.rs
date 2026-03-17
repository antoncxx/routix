use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{context::Context, proxy::ProxyHost};
use async_trait::async_trait;
use pingora::prelude::*;

pub(crate) struct RoutixProxy {
    context: Context,
    counter: AtomicUsize,
}

pub(crate) struct RequestContext {
    pub proxy_host: Arc<ProxyHost>,
    pub upstream_index: usize,
}

#[async_trait]
impl ProxyHttp for RoutixProxy {
    type CTX = Option<RequestContext>;

    fn new_ctx(&self) -> Self::CTX {
        None
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let proxy_host = self
            .resolve_host(session)
            .await
            .ok_or_else(|| Error::new(ErrorType::HTTPStatus(502)))?;

        let index = self.counter.fetch_add(1, Ordering::Relaxed);

        let upstream = proxy_host
            .select_upstream(index)
            .ok_or_else(|| Error::new(ErrorType::HTTPStatus(502)))?;

        let peer = upstream.to_peer();

        *ctx = Some(RequestContext {
            proxy_host,
            upstream_index: index,
        });

        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()> {
        let Some(req_ctx) = ctx else { return Ok(()) };

        let upstream = req_ctx
            .proxy_host
            .select_upstream(req_ctx.upstream_index)
            .ok_or_else(|| Error::new(ErrorType::HTTPStatus(502)))?;

        upstream_request.insert_header("host", upstream.host_header())?;

        Ok(())
    }
}

impl RoutixProxy {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            counter: AtomicUsize::new(0),
        }
    }

    async fn resolve_host(&self, session: &Session) -> Option<Arc<ProxyHost>> {
        let hostname = session
            .req_header()
            .headers
            .get("host")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(':').next())?;

        self.context.hosts_manager.get(hostname).await
    }
}
