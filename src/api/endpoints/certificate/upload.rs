use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use base64::{Engine, engine::general_purpose::STANDARD};
use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

use crate::context::Context;
use crate::database::models::NewCertificateModel;
use crate::database::repos::CertificatesRepository;
use crate::tls::Certificate;

static CERT_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_.-]+$").unwrap());

#[derive(Deserialize, Validate)]
pub struct UploadCertificateRequestBody {
    #[validate(length(min = 1), regex(path = *CERT_NAME_REGEX))]
    name: String,

    #[validate(length(min = 1))]
    certificate: String,

    #[validate(length(min = 1))]
    pem_key: String,
}

pub async fn upload(
    State(ctx): State<Context>,
    Json(body): Json<UploadCertificateRequestBody>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let (Ok(Ok(cert_pem)), Ok(Ok(key_pem))) = (
        STANDARD.decode(&body.certificate).map(String::from_utf8),
        STANDARD.decode(&body.pem_key).map(String::from_utf8),
    ) else {
        return StatusCode::BAD_REQUEST.into_response();
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
        type_: "custom".to_owned(),
        dns_provider: None,
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
