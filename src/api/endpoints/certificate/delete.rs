use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::{context::Context, database::repos::CertificatesRepository};

pub async fn delete(State(ctx): State<Context>, Path(id): Path<i32>) -> impl IntoResponse {
    let model = match CertificatesRepository::fetch(id, &ctx.database).await {
        Ok(model) => model,
        Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match CertificatesRepository::delete(id, &ctx.database).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) if e.is_not_found() => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    ctx.hosts_manager.remove(&model.name).await;

    StatusCode::NO_CONTENT.into_response()
}
