use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::database::models::ProxyHostModel;
use crate::{context::Context, database::repos::ProxyHostsRepository};

#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    page: i64,
    #[serde(default = "default_per_page")]
    per_page: i64,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    20
}

#[derive(Serialize)]
pub struct PaginatedResponse {
    data: Vec<ProxyHostModel>,
    total: i64,
    page: i64,
    per_page: i64,
    total_pages: i64,
}

pub async fn fetch_all(
    State(ctx): State<Context>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    match ProxyHostsRepository::get_all(&ctx.database, pagination.page, pagination.per_page).await {
        Ok((hosts, total)) => Json(PaginatedResponse {
            data: hosts,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages: (total + pagination.per_page - 1) / pagination.per_page,
        })
        .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
