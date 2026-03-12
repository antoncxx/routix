use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::api::endpoints::utils::PaginatedResponse;
use crate::{context::Context, database::repos::CertificatesRepository};

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

pub async fn get_all(
    State(ctx): State<Context>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    match CertificatesRepository::get_all(&ctx.database, pagination.page, pagination.per_page).await
    {
        Ok((certs, total)) => Json(PaginatedResponse {
            data: certs,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages: (total + pagination.per_page - 1) / pagination.per_page,
        })
        .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
