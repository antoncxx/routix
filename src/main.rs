use crate::{api::ApiConfig, context::Context};

mod api;
mod context;
mod database;
mod init;
mod jwt;
mod repos;
mod roles;
mod scopes;
mod tls;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();

    let context = match Context::new() {
        Ok(ctx) => ctx,
        Err(err) => {
            log::error!("Failed to initialize application context:\n{err}");
            return;
        }
    };

    let api_config = match ApiConfig::from_env() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to read Api Configuration:\n{err}");
            return;
        }
    };

    if let Err(err) = init::initialize(context.clone()).await {
        log::error!("Failed to initiaze the environment:\n{err}");
        return;
    }

    api::run_rest_api(context, api_config).await.unwrap();
}
