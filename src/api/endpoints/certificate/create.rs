use crate::context::Context;
use crate::database::models::NewCertificateModel;
use crate::database::repos::CertificatesRepository;
use crate::tls::Certificate;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCertificateRequestBody {
    certificate: String,
    pem_key: String,
    name: String,
}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateCertificateRequestBody>,
) -> impl IntoResponse {
    let (cert_pem, key_pem) = match (
        STANDARD.decode(&body.certificate).map(String::from_utf8),
        STANDARD.decode(&body.pem_key).map(String::from_utf8),
    ) {
        (Ok(Ok(cert)), Ok(Ok(key))) => (cert, key),
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };

    let Ok(certificate) = Certificate::new(&cert_pem, &key_pem) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let Ok(private_key) = ctx.certificates_manager.encrypt_certificate_key(&key_pem) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let model = NewCertificateModel {
        private_key,
        name: body.name.clone(),
        certificate: body.certificate,
        expires_at: certificate.expires_at().ok(),
        ..Default::default()
    };

    match CertificatesRepository::create(model, &ctx.database).await {
        Ok(_) => {
            let _ = ctx.certificates_manager.add(&body.name, certificate).await;
            StatusCode::CREATED.into_response()
        }
        Err(e) if e.is_unique_violation() => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
