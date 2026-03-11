use crate::api::middleware::{auth_middleware, scoped_middleware};
use crate::context::Context;
use crate::scopes::UserScope;
use axum::{Router, middleware, routing::post};

mod request;
mod upload;

pub fn router(context: Context) -> Router<Context> {
    Router::new()
        .route(
            "/upload",
            post(upload::upload)
                .route_layer(middleware::from_fn_with_state(
                    UserScope::CertificatesWrite,
                    scoped_middleware,
                ))
                .route_layer(middleware::from_fn_with_state(
                    context.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/request",
            post(request::request)
                .route_layer(middleware::from_fn_with_state(
                    UserScope::CertificatesWrite,
                    scoped_middleware,
                ))
                .route_layer(middleware::from_fn_with_state(
                    context.clone(),
                    auth_middleware,
                )),
        )
        .with_state(context)
}
