use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use validator::Validate;

use crate::database::models::NewProxyHostModel;
use crate::proxy::ProxyHost;
use crate::{context::Context, database::repos::ProxyHostsRepository};

use crate::api::endpoints::utils::{DOMAIN_REGEX, HOST_REGEX, validate_forward_schema};

#[derive(Deserialize, Validate)]
pub struct CreateProxyHostRequest {
    #[validate(length(min = 1, max = 255), regex(path = *DOMAIN_REGEX))]
    domain: String,
    #[validate(custom(function = "validate_forward_schema"))]
    forward_schema: String,
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
            forward_schema: value.forward_schema,
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

    let model = match ProxyHostsRepository::create(body.into(), &ctx.database).await {
        Ok(model) => model,
        Err(e) if e.is_unique_violation() => return StatusCode::CONFLICT.into_response(),
        Err(e) if e.is_foreign_key_violation() => return StatusCode::BAD_REQUEST.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let Ok(proxy_host) = ProxyHost::try_from(model) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    ctx.hosts_manager.add(proxy_host).await;
    StatusCode::CREATED.into_response()
}
