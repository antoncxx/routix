use std::net::IpAddr;
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

    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        let proxy_host = self
            .resolve_host(session)
            .await
            .ok_or_else(|| Error::new(ErrorType::HTTPStatus(502)))?;

        let client_ip: IpAddr = session
            .client_addr()
            .and_then(|a| a.as_inet())
            .map(std::net::SocketAddr::ip)
            .ok_or_else(|| Error::new(ErrorType::InternalError))?;

        if !proxy_host.is_allowed(&client_ip) {
            let mut resp = ResponseHeader::build(403, None)?;
            resp.insert_header("content-length", "0")?;
            session.write_response_header(Box::new(resp), true).await?;
            return Ok(true);
        }

        *ctx = Some(RequestContext {
            proxy_host,
            upstream_index: 0, // set properly in upstream_peer
        });

        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let ctx = ctx
            .as_mut()
            .ok_or_else(|| Error::new(ErrorType::HTTPStatus(502)))?;

        let index = self.counter.fetch_add(1, Ordering::Relaxed);
        ctx.upstream_index = index;

        let peer = ctx
            .proxy_host
            .select_upstream(index)
            .ok_or_else(|| Error::new(ErrorType::HTTPStatus(502)))?
            .to_peer();

        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()> {
        let ctx = ctx
            .as_ref()
            .ok_or_else(|| Error::new(ErrorType::HTTPStatus(502)))?;

        let upstream = ctx
            .proxy_host
            .select_upstream(ctx.upstream_index)
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
