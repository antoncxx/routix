use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::context::Context;
use crate::database::repos::AccessListsRepository;

pub async fn delete(State(ctx): State<Context>, Path(id): Path<i32>) -> impl IntoResponse {
    match AccessListsRepository::delete(id, &ctx.database).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) if e.is_not_found() => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
