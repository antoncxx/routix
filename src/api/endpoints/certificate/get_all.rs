use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use validator::Validate;

use crate::api::endpoints::utils::{PaginatedResponse, PaginationQuery};
use crate::database::models::CertificateModel;
use crate::{context::Context, database::repos::CertificatesRepository};

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
    if pagination.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

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
