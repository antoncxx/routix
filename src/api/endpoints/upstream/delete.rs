use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::{context::Context, database::repos::UpstreamsRepository};

pub async fn delete(State(ctx): State<Context>, Path(id): Path<i32>) -> impl IntoResponse {
    match UpstreamsRepository::delete(id, &ctx.database).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) if e.is_not_found() => StatusCode::NOT_FOUND.into_response(),
        Err(e) if e.is_foreign_key_violation() => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
