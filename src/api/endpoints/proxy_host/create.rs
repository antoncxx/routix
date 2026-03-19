use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use validator::Validate;

use crate::database::models::NewProxyHostModel;
use crate::proxy::ProxyHost;
use crate::{
    context::Context,
    database::repos::{ProxyHostsRepository, UpstreamsRepository},
};

use crate::api::endpoints::utils::DOMAIN_REGEX;

#[derive(Deserialize, Validate)]
pub struct CreateProxyHostRequest {
    #[validate(length(min = 1, max = 255), regex(path = *DOMAIN_REGEX))]
    domain: String,
    certificate_name: Option<String>,
    access_list_id: Option<i32>,
    upstream_ids: Vec<i32>,
}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateProxyHostRequest>,
) -> impl IntoResponse {
    if body.validate().is_err() || body.upstream_ids.is_empty() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let model = NewProxyHostModel {
        domain: body.domain,
        certificate_name: body.certificate_name,
        access_list_id: body.access_list_id,
    };

    let host_model =
        match ProxyHostsRepository::create(model, body.upstream_ids.clone(), &ctx.database).await {
            Ok(model) => model,
            Err(e) if e.is_unique_violation() => return StatusCode::CONFLICT.into_response(),
            Err(e) if e.is_foreign_key_violation() => {
                return StatusCode::BAD_REQUEST.into_response();
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

    let Ok(upstream_models) =
        UpstreamsRepository::get_by_ids(body.upstream_ids, &ctx.database).await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(proxy_host) = ProxyHost::new(host_model, upstream_models) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    ctx.hosts_manager.add(proxy_host).await;
    StatusCode::CREATED.into_response()
}
