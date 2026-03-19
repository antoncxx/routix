use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Deserializer};
use validator::Validate;

use crate::database::models::UpdateProxyHostModel;
use crate::proxy::ProxyHost;
use crate::{
    context::Context,
    database::repos::{ProxyHostUpstreamsRepository, ProxyHostsRepository, UpstreamsRepository},
};

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

impl UpdateProxyHostRequest {
    pub fn has_changes(&self) -> bool {
        self.domain.is_some()
    }

    pub fn to_model(&self) -> UpdateProxyHostModel {
        UpdateProxyHostModel {
            domain: self.domain.clone(),
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

    let cert_name = body.certificate_name.clone();
    let access_list_id = body.access_list_id;

    let host_model = if body.has_changes() {
        match ProxyHostsRepository::update(id, body.to_model(), &ctx.database).await {
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

    let host_model = if let Some(cert) = cert_name {
        match ProxyHostsRepository::update_certificate(id, cert, &ctx.database).await {
            Ok(model) => model,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        host_model
    };

    let host_model = if let Some(access_list) = access_list_id {
        match ProxyHostsRepository::update_access_list(id, access_list, &ctx.database).await {
            Ok(model) => model,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        host_model
    };

    let upstream_ids =
        match ProxyHostUpstreamsRepository::get_by_proxy_host(id, &ctx.database).await {
            Ok(upstreams) => upstreams.into_iter().map(|u| u.id).collect(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

    let Ok(upstream_models) = UpstreamsRepository::get_by_ids(upstream_ids, &ctx.database).await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(proxy_host) = ProxyHost::new(host_model, upstream_models) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    ctx.hosts_manager.update(proxy_host).await;
    StatusCode::OK.into_response()
}

// @TODO: Add fetch list and rules and add them to ProxyHost model
