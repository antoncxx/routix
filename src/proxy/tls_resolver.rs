use async_trait::async_trait;
use pingora::listeners::TlsAccept;
use pingora::protocols::tls::TlsRef;
use pingora_openssl::ext;
use std::sync::Arc;

use crate::context::Context;
use crate::tls::Certificate;

pub struct TlsResolver {
    context: Context,
}

#[async_trait]
impl TlsAccept for TlsResolver {
    async fn certificate_callback(&self, ssl: &mut TlsRef) -> () {
        let hostname = ssl.servername(openssl::ssl::NameType::HOST_NAME);

        let Some(hostname) = hostname else { return };
        let Some(certificate) = self.resolve_certificate(hostname).await else {
            return;
        };

        let _ = ext::ssl_use_certificate(ssl, &certificate.certificate);
        let _ = ext::ssl_use_private_key(ssl, &certificate.private_key);
    }
}

impl TlsResolver {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    async fn resolve_certificate(&self, hostname: &str) -> Option<Arc<Certificate>> {
        let proxy_host = self.context.hosts_manager.get(hostname).await?;

        if !proxy_host.is_https() {
            return None;
        }

        let cert_name = proxy_host.certificate_name.as_ref()?;
        self.context.certificates_manager.get(cert_name).await
    }
}
