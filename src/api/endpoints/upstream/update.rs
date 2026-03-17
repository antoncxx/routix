use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use validator::Validate;

use crate::api::endpoints::utils::{HOST_REGEX, validate_forward_schema};
use crate::database::models::UpdateUpstreamModel;
use crate::{context::Context, database::repos::UpstreamsRepository};

#[derive(Deserialize, Validate)]
pub struct UpdateUpstreamRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 255), regex(path = *HOST_REGEX))]
    pub host: Option<String>,
    #[validate(range(min = 1, max = 65535))]
    pub port: Option<i32>,
    #[validate(custom(function = "validate_forward_schema"))]
    pub schema: Option<String>,
}

pub async fn update(
    State(ctx): State<Context>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateUpstreamRequest>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    if let Some(ref schema) = body.schema
        && !matches!(schema.as_str(), "http" | "https")
    {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let model = UpdateUpstreamModel {
        name: body.name,
        host: body.host,
        port: body.port,
        schema: body.schema,
    };

    match UpstreamsRepository::update(id, model, &ctx.database).await {
        Ok(upstream) => Json(upstream).into_response(),
        Err(e) if e.is_unique_violation() => StatusCode::CONFLICT.into_response(),
        Err(e) if e.is_not_found() => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// @TODO: update proxy hosts that reference this upstream
