use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use validator::Validate;

use crate::api::endpoints::utils::{PaginatedResponse, PaginationQuery};
use crate::database::models::{ProxyHostModel, UpstreamModel};
use crate::database::repos::ProxyHostUpstreamsRepository;
use crate::{context::Context, database::repos::ProxyHostsRepository};

#[derive(Serialize)]
pub struct ProxyHostResponse {
    #[serde(flatten)]
    pub host: ProxyHostModel,
    pub upstreams: Vec<UpstreamModel>,
    pub access_list: Option<String>,
}

pub async fn fetch_all(
    State(ctx): State<Context>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    if pagination.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let Ok(total) = ProxyHostsRepository::count(&ctx.database).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(data) = ProxyHostUpstreamsRepository::get_all_with_upstreams(
        &ctx.database,
        pagination.page,
        pagination.per_page,
    )
    .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let response_items = data
        .into_iter()
        .map(|(host, upstreams, access_list_data)| ProxyHostResponse {
            host,
            upstreams,
            access_list: access_list_data.map(|data| data.0.name),
        })
        .collect::<Vec<_>>();

    Json(PaginatedResponse {
        data: response_items,
        total,
        page: pagination.page,
        per_page: pagination.per_page,
        total_pages: (total + pagination.per_page - 1) / pagination.per_page,
    })
    .into_response()
}
