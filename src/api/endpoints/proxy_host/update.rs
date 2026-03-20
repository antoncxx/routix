use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Deserializer};
use validator::Validate;

use crate::database::models::{UpdateProxyHost, UpdateProxyHostModel};
use crate::proxy::ProxyHost;
use crate::{context::Context, database::repos::ProxyHostsRepository};

use crate::api::endpoints::utils::DOMAIN_REGEX;

#[derive(Deserialize, Validate)]
pub struct UpdateProxyHostRequest {
    #[validate(length(min = 1, max = 255), regex(path = *DOMAIN_REGEX))]
    domain: Option<String>,
    #[allow(clippy::option_option)]
    #[serde(default, deserialize_with = "deserialize_certificate_name")]
    pub certificate_name: Option<Option<String>>,
    #[allow(clippy::option_option)]
    #[serde(default, deserialize_with = "deserialize_access_list_id")]
    pub access_list_id: Option<Option<i32>>,
    pub upstream_ids: Option<Vec<i32>>,
}

#[allow(clippy::option_option)]
fn deserialize_certificate_name<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

#[allow(clippy::option_option)]
fn deserialize_access_list_id<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}

impl From<UpdateProxyHostRequest> for UpdateProxyHost {
    fn from(value: UpdateProxyHostRequest) -> Self {
        UpdateProxyHost {
            model: UpdateProxyHostModel {
                domain: value.domain,
                certificate_name: value.certificate_name,
                access_list_id: value.access_list_id,
            },
            upstream_ids: value.upstream_ids,
        }
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

    if let Some(ref ids) = body.upstream_ids
        && ids.is_empty()
    {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let model = body.into();

    let (host_model, upstream_models, access_list) =
        match ProxyHostsRepository::update_full(id, model, &ctx.database).await {
            Ok(result) => result,
            Err(e) if e.is_unique_violation() => return StatusCode::CONFLICT.into_response(),
            Err(e) if e.is_foreign_key_violation() => {
                return StatusCode::BAD_REQUEST.into_response();
            }
            Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

    let Ok(proxy_host) = ProxyHost::new(host_model, upstream_models, access_list) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    ctx.hosts_manager.update(proxy_host).await;
    StatusCode::OK.into_response()
}
