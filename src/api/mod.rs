mod config;
mod endpoints;
mod middleware;

pub use config::*;

use crate::context::Context;

pub async fn run_rest_api(context: Context, config: ApiConfig) -> Result<(), std::io::Error> {
    let router = endpoints::build_router(context);

    let listener = tokio::net::TcpListener::bind(config.addr).await?;

    log::info!("REST API listening on http://{}", config.addr);

    axum::serve(listener, router).await
}
