use crate::api::middleware::{admin_middleware, auth_middleware};
use crate::context::Context;
use axum::{
    Router, middleware,
    routing::{patch, post},
};

mod create;
mod update;

pub fn router(context: Context) -> Router<Context> {
    Router::new()
        .route(
            "/",
            post(create::create)
                .route_layer(middleware::from_fn(admin_middleware))
                .route_layer(middleware::from_fn_with_state(
                    context.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/{username}",
            patch(update::update)
                .route_layer(middleware::from_fn(admin_middleware))
                .route_layer(middleware::from_fn_with_state(
                    context.clone(),
                    auth_middleware,
                )),
        )
        .with_state(context)
}
