use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use validator::Validate;

use crate::api::endpoints::utils::{HOST_REGEX, validate_forward_schema};
use crate::database::models::NewUpstreamModel;
use crate::{context::Context, database::repos::UpstreamsRepository};

#[derive(Deserialize, Validate)]
pub struct CreateUpstreamRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 255), regex(path = *HOST_REGEX))]
    pub host: String,
    #[validate(range(min = 1, max = 65535))]
    pub port: i32,
    #[validate(custom(function = "validate_forward_schema"))]
    pub schema: Option<String>,
}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateUpstreamRequest>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let schema = body.schema.unwrap_or_else(|| "http".to_string());
    if !matches!(schema.as_str(), "http" | "https") {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let model = NewUpstreamModel {
        name: body.name,
        host: body.host,
        port: body.port,
        schema,
    };

    match UpstreamsRepository::create(model, &ctx.database).await {
        Ok(upstream) => Json(upstream).into_response(),
        Err(e) if e.is_unique_violation() => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
