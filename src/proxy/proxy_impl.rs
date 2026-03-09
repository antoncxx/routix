use std::sync::Arc;

use crate::{context::Context, proxy::ProxyHost};
use async_trait::async_trait;
use pingora::prelude::*;

pub(crate) struct RoutixProxy {
    context: Context,
}

pub(crate) struct RequestContext {
    pub proxy_host: Arc<ProxyHost>,
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
        let hostname = session
            .req_header()
            .headers
            .get("host")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(':').next())
            .unwrap_or("");

        let Some(proxy_host) = self.context.hosts_manager.get(hostname).await else {
            return Err(Error::new(ErrorType::HTTPStatus(502)));
        };

        let upstream = proxy_host.upstream();
        *ctx = Some(RequestContext { proxy_host });

        Ok(upstream)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()> {
        let Some(req_ctx) = ctx else { return Ok(()) };

        upstream_request.insert_header("host", req_ctx.proxy_host.upstream_host_header())?;

        Ok(())
    }
}

impl RoutixProxy {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}
