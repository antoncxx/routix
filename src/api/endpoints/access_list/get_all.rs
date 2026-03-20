use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use validator::Validate;

use crate::api::endpoints::utils::{PaginatedResponse, PaginationQuery};
use crate::context::Context;
use crate::database::models::{AccessListModel, AccessListRuleModel};
use crate::database::repos::AccessListsRepository;

#[derive(Serialize)]
pub struct AccessListResponse {
    #[serde(flatten)]
    pub access_list: AccessListModel,
    pub rules: Vec<AccessListRuleModel>,
}

pub async fn fetch_all(
    State(ctx): State<Context>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    if pagination.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let Ok(total) = AccessListsRepository::count(&ctx.database).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(data) =
        AccessListsRepository::fetch_all(&ctx.database, pagination.page, pagination.per_page).await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let response_items = data
        .into_iter()
        .map(|(access_list, rules)| AccessListResponse { access_list, rules })
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
