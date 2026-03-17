use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use validator::Validate;

use crate::api::endpoints::utils::{PaginatedResponse, PaginationQuery};
use crate::{context::Context, database::repos::UpstreamsRepository};

pub async fn fetch_all(
    State(ctx): State<Context>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    if pagination.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    match UpstreamsRepository::get_all(&ctx.database, pagination.page, pagination.per_page).await {
        Ok((items, total)) => Json(PaginatedResponse {
            data: items,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages: (total + pagination.per_page - 1) / pagination.per_page,
        })
        .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
