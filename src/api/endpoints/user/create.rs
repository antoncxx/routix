use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::context::Context;

#[derive(Deserialize)]
pub struct CreateUserRequestbody {}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateUserRequestbody>,
) -> impl IntoResponse {
    let _ = ctx;
    let _ = body;

    StatusCode::OK
}
