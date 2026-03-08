use crate::api::middleware::{auth_middleware, scoped_middleware};
use crate::context::Context;
use crate::scopes::UserScope;
use axum::{Router, middleware, routing::post};

mod create;

pub fn router(context: Context) -> Router<Context> {
    Router::new()
        .route(
            "/",
            post(create::create)
                .route_layer(middleware::from_fn_with_state(
                    UserScope::ProxyHostsWrite,
                    scoped_middleware,
                ))
                .route_layer(middleware::from_fn_with_state(
                    context.clone(),
                    auth_middleware,
                )),
        )
        .with_state(context)
}
