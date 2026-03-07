use axum::{Router, routing::post};

use crate::context::Context;

mod certificate;
mod login;
mod user;

pub fn build_router(context: Context) -> Router {
    Router::new()
        .route("/login", post(login::login))
        .nest("/user", user::router(context.clone()))
        .nest("/certificate", certificate::router(context.clone()))
        .with_state(context)
}
