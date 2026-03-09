use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

use crate::database::models::NewProxyHostModel;
use crate::{context::Context, database::repos::ProxyHostsRepository};

static DOMAIN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap()
});

static HOST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9-]+$").unwrap()
});

#[derive(Deserialize, Validate)]
pub struct CreateProxyHostRequest {
    #[validate(length(min = 1, max = 255), regex(path = *DOMAIN_REGEX))]
    domain: String,
    #[validate(length(min = 1, max = 255), regex(path = *HOST_REGEX))]
    forward_host: String,
    #[validate(range(min = 1, max = 65535))]
    forward_port: i32,
    certificate_name: Option<String>,
}

impl From<CreateProxyHostRequest> for NewProxyHostModel {
    fn from(value: CreateProxyHostRequest) -> Self {
        Self {
            domain: value.domain,
            forward_host: value.forward_host,
            forward_port: value.forward_port,
            certificate_name: value.certificate_name,
        }
    }
}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateProxyHostRequest>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    match ProxyHostsRepository::create(body.into(), &ctx.database).await {
        Ok(_) => {
            // TODO: Add host to proxy's context
            StatusCode::CREATED.into_response()
        }
        Err(e) if e.is_unique_violation() => StatusCode::CONFLICT.into_response(),
        Err(e) if e.is_foreign_key_violation() => StatusCode::BAD_REQUEST.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
