use crate::{api::ApiConfig, context::Context};

mod api;
mod context;
mod jwt;

#[tokio::main]
async fn main() {
    env_logger::init();

    let context = match Context::new() {
        Ok(ctx) => ctx,
        Err(err) => {
            log::error!("Failed to initialize application context: {err}");
            return;
        }
    };

    let api_config = match ApiConfig::from_env() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to read Api Configuration: {err}");
            return;
        }
    };

    api::run_rest_api(context, api_config).await.unwrap()
}
