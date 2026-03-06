use crate::{api::ApiConfig, context::Context};

mod api;
mod context;
mod database;
mod init;
mod jwt;
mod repos;
mod roles;
mod scopes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
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

    if init::initialize(context.clone()).await.is_err() {
        log::error!("Failed to initiaze the environment");
        return;
    }

    api::run_rest_api(context, api_config).await.unwrap();
}
