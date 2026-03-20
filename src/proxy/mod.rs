use crate::context::Context;
use pingora::{listeners::tls::TlsSettings, prelude::*};
use proxy_impl::RoutixProxy;

mod accees_list;
mod hosts_manager;
mod proxy_host;
mod proxy_impl;
mod tls_resolver;
mod upstream;

pub use hosts_manager::*;
pub use proxy_host::*;

#[allow(clippy::similar_names)]
pub async fn run_proxy(context: Context) -> anyhow::Result<()> {
    tokio::task::spawn_blocking(move || {
        let mut server = Server::new(None).map_err(|e| anyhow::anyhow!(e))?;
        server.bootstrap();

        let mut http_proxy =
            http_proxy_service(&server.configuration, RoutixProxy::new(context.clone()));

        http_proxy.add_tcp("0.0.0.0:80");

        let mut https_proxy =
            http_proxy_service(&server.configuration, RoutixProxy::new(context.clone()));

        let tls_settings =
            TlsSettings::with_callbacks(Box::new(tls_resolver::TlsResolver::new(context)))
                .map_err(|e| anyhow::anyhow!(e))?;

        https_proxy.add_tls_with_settings("0.0.0.0:443", None, tls_settings);

        server.add_service(http_proxy);
        server.add_service(https_proxy);

        server.run_forever()
    })
    .await?
}
