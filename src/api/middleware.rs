use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{context::Context, jwt::Claims, scopes::UserScope};

pub async fn auth_middleware(State(ctx): State<Context>, mut req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match token {
        Some(t) => match ctx.jwt.verify(t) {
            Ok(data) => {
                req.extensions_mut().insert(data.claims);
                next.run(req).await
            }
            Err(_) => StatusCode::UNAUTHORIZED.into_response(),
        },
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

pub async fn admin_middleware(req: Request, next: Next) -> Response {
    let claims = req.extensions().get::<Claims>();

    match claims {
        Some(claims) if claims.is_admin() => next.run(req).await,
        Some(_) => StatusCode::FORBIDDEN.into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

pub async fn scoped_middleware(
    State(scope): State<UserScope>,
    req: Request,
    next: Next,
) -> Response {
    let claims = req.extensions().get::<Claims>();

    match claims {
        Some(claims) if claims.is_admin() || claims.has_scope(scope) => next.run(req).await,
        Some(_) => StatusCode::FORBIDDEN.into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}
