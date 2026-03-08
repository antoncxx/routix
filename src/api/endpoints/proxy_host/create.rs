use crate::database::models::NewProxyHostModel;
use crate::{context::Context, database::repos::ProxyHostsRepository};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateProxyHostRequest {
    domain: String,
    forward_host: String,
    forward_port: i32,
    certificate_name: Option<String>,
}

impl From<CreateProxyHostRequest> for NewProxyHostModel {
    fn from(value: CreateProxyHostRequest) -> Self {
        Self {
            domain: value.domain,
            forward_host: value.forward_host,
            forward_port: value.forward_port,
            certificate_name: value.certificate_name,
        }
    }
}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateProxyHostRequest>,
) -> impl IntoResponse {
    match ProxyHostsRepository::create(body.into(), &ctx.database).await {
        Ok(_) => {
            // TODO: Add host to proxy's context
            StatusCode::CREATED.into_response()
        }
        Err(e) if e.is_unique_violation() => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
