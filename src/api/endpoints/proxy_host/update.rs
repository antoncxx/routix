use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Deserializer};
use validator::Validate;

use crate::database::models::UpdateProxyHostModel;
use crate::proxy::ProxyHost;
use crate::{context::Context, database::repos::ProxyHostsRepository};

use super::utils::{DOMAIN_REGEX, HOST_REGEX, validate_forward_schema};

#[derive(Deserialize, Validate)]
pub struct UpdateProxyHostRequest {
    #[validate(length(min = 1, max = 255), regex(path = *DOMAIN_REGEX))]
    domain: Option<String>,
    #[validate(custom(function = "validate_forward_schema"))]
    forward_schema: Option<String>,
    #[validate(length(min = 1, max = 255), regex(path = *HOST_REGEX))]
    forward_host: Option<String>,
    #[validate(range(min = 1, max = 65535))]
    forward_port: Option<i32>,
    #[allow(clippy::option_option)]
    #[serde(default, deserialize_with = "deserialize_certificate_name")]
    pub certificate_name: Option<Option<String>>,
}

#[allow(clippy::option_option)]
fn deserialize_certificate_name<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

impl From<UpdateProxyHostRequest> for UpdateProxyHostModel {
    fn from(value: UpdateProxyHostRequest) -> Self {
        Self {
            domain: value.domain,
            forward_schema: value.forward_schema,
            forward_host: value.forward_host,
            forward_port: value.forward_port,
        }
    }
}

impl UpdateProxyHostRequest {
    pub fn has_changes(&self) -> bool {
        self.domain.is_some()
            || self.forward_schema.is_some()
            || self.forward_host.is_some()
            || self.forward_port.is_some()
    }
}

pub async fn update(
    State(ctx): State<Context>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateProxyHostRequest>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let cert_name = body.certificate_name.clone();

    let model = if body.has_changes() {
        match ProxyHostsRepository::update(id, body.into(), &ctx.database).await {
            Ok(model) => model,
            Err(e) if e.is_unique_violation() => return StatusCode::CONFLICT.into_response(),
            Err(e) if e.is_foreign_key_violation() => {
                return StatusCode::BAD_REQUEST.into_response();
            }
            Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        match ProxyHostsRepository::fetch(id, &ctx.database).await {
            Ok(model) => model,
            Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    };

    let model = if let Some(cert) = cert_name {
        match ProxyHostsRepository::update_certificate(id, cert, &ctx.database).await {
            Ok(model) => model,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        model
    };

    let Ok(proxy_host) = ProxyHost::try_from(model) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    ctx.hosts_manager.update(proxy_host).await;
    StatusCode::OK.into_response()
}
