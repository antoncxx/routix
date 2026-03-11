use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use base64::{Engine, engine::general_purpose::STANDARD};
use regex::Regex;
use serde::Deserialize;
use std::str::FromStr;
use std::sync::LazyLock;
use validator::Validate;

use crate::cert::{self, DnsProviderId};
use crate::context::Context;
use crate::database::models::NewCertificateModel;
use crate::database::repos::CertificatesRepository;

static DOMAIN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap()
});

const DEFAULT_DNS_PROPAGATION_SECS: u64 = 30;

#[derive(Deserialize, Validate)]
pub struct CreateCertificateRequestBody {
    #[validate(length(min = 1, max = 255), regex(path = *DOMAIN_REGEX))]
    domain: String,

    #[validate(length(min = 1))]
    dns_provider: String,

    #[validate(range(min = 1, max = 300))]
    dns_propagation_secs: Option<u64>,
}

pub async fn request(
    State(ctx): State<Context>,
    Json(body): Json<CreateCertificateRequestBody>,
) -> StatusCode {
    handle_request(ctx, body).await.unwrap_or_else(|e| e)
}

async fn handle_request(
    ctx: Context,
    body: CreateCertificateRequestBody,
) -> Result<StatusCode, StatusCode> {
    body.validate()
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    if ctx.certificates_manager.get(&body.domain).await.is_some() {
        return Ok(StatusCode::NO_CONTENT);
    }

    let provider_id =
        DnsProviderId::from_str(&body.dns_provider).map_err(|_| StatusCode::BAD_REQUEST)?;

    let dns_provider =
        cert::create_dns_provider(provider_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let certificate = ctx
        .certificate_authority
        .request_certificate(
            &body.domain,
            dns_provider.as_ref(),
            body.dns_propagation_secs
                .unwrap_or(DEFAULT_DNS_PROPAGATION_SECS),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let model = build_certificate_model(&ctx, &body.domain, &certificate, &body.dns_provider)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match CertificatesRepository::create(model, &ctx.database).await {
        Ok(_) => {
            let _ = ctx
                .certificates_manager
                .add(&body.domain, certificate)
                .await;
            Ok(StatusCode::CREATED)
        }
        Err(e) if e.is_unique_violation() => Err(StatusCode::CONFLICT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn build_certificate_model(
    ctx: &Context,
    domain: &str,
    certificate: &crate::tls::Certificate,
    dns_provider: &str,
) -> anyhow::Result<NewCertificateModel> {
    let pem_bytes = certificate.private_key.private_key_to_pem_pkcs8()?;
    let pem_string = String::from_utf8(pem_bytes)?;
    let encrypted_key = ctx
        .certificates_manager
        .encrypt_certificate_key(&pem_string)?;

    let cert_pem = certificate.certificate.to_pem()?;
    let cert_pem_string = String::from_utf8(cert_pem)?;
    let certificate_pem_base64 = STANDARD.encode(cert_pem_string);

    Ok(NewCertificateModel {
        private_key: encrypted_key,
        name: domain.to_string(),
        certificate: certificate_pem_base64,
        expires_at: certificate.expires_at().ok(),
        dns_provider: Some(dns_provider.to_owned()),
        type_: "letsencrypt".to_owned(),
    })
}
