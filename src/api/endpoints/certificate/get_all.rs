use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::api::endpoints::utils::PaginatedResponse;
use crate::database::models::CertificateModel;
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

#[derive(Serialize)]
pub struct CertificateReturnValue {
    pub id: i32,
    pub name: String,
    pub r#type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<CertificateModel> for CertificateReturnValue {
    fn from(model: CertificateModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            r#type: model.type_,
            created_at: model.created_at,
            expires_at: model.expires_at,
        }
    }
}

pub async fn get_all(
    State(ctx): State<Context>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    match CertificatesRepository::get_all(&ctx.database, pagination.page, pagination.per_page).await
    {
        Ok((certs, total)) => Json(PaginatedResponse {
            data: certs
                .into_iter()
                .map(CertificateReturnValue::from)
                .collect(),
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages: (total + pagination.per_page - 1) / pagination.per_page,
        })
        .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
