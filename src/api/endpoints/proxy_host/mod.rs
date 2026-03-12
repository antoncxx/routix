use crate::api::middleware::{auth_middleware, scoped_middleware};
use crate::context::Context;
use crate::scopes::UserScope;
use axum::{
    Router, middleware,
    routing::{get, post, put},
};

mod create;
mod get_all;
mod update;
mod utils;
pub fn router(context: Context) -> Router<Context> {
    Router::new()
        .route(
            "/",
            get(get_all::fetch_all)
                .route_layer(middleware::from_fn_with_state(
                    UserScope::ProxyHostsRead,
                    scoped_middleware,
                ))
                .route_layer(middleware::from_fn_with_state(
                    context.clone(),
                    auth_middleware,
                )),
        )
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
        .route(
            "/:id",
            put(update::update)
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
