use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::{context::Context, database::repos::ProxyHostsRepository};

pub async fn delete(State(ctx): State<Context>, Path(id): Path<i32>) -> impl IntoResponse {
    let model = match ProxyHostsRepository::fetch(id, &ctx.database).await {
        Ok(model) => model,
        Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match ProxyHostsRepository::delete(id, &ctx.database).await {
        Ok(()) => {}
        Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }

    ctx.hosts_manager.remove(&model.domain).await;

    StatusCode::NO_CONTENT.into_response()
}
