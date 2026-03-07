use std::sync::Arc;

use chrono::{DateTime, Utc};
use openssl::asn1::Asn1Time;
use pingora_openssl::error::ErrorStack;
use pingora_openssl::pkey::{PKey, Private};
use pingora_openssl::ssl::{SslContext, SslContextBuilder, SslMethod};
use pingora_openssl::x509::X509;

#[derive(Debug)]
pub struct Certificate {
    certificate: X509,
    private_key: PKey<Private>,
    ssl_context: Arc<SslContext>,
}

impl Certificate {
    pub fn new(cert_pem: &str, key_pem: &str) -> Result<Self, ErrorStack> {
        let certificate = X509::from_pem(cert_pem.as_bytes())?;
        let private_key = PKey::private_key_from_pem(key_pem.as_bytes())?;

        let mut builder = SslContextBuilder::new(SslMethod::tls())?;
        builder.set_certificate(&certificate)?;
        builder.set_private_key(&private_key)?;
        builder.check_private_key()?;

        Ok(Self {
            certificate,
            private_key,
            ssl_context: builder.build().into(),
        })
    }

    pub fn expires_at(&self) -> Result<DateTime<Utc>, ErrorStack> {
        let not_after = self.certificate.not_after();
        let epoch = Asn1Time::from_unix(0)?;
        let diff = epoch.diff(not_after)?;

        let secs = diff.days as i64 * 86_400 + diff.secs as i64;

        Ok(DateTime::from_timestamp(secs, 0).unwrap_or_default())
    }

    pub fn get_context(&self) -> Arc<SslContext> {
        self.ssl_context.clone()
    }

    pub fn hostnames(&self) -> Vec<String> {
        let mut hostnames = Vec::new();

        if let Some(san) = self.certificate.subject_alt_names() {
            for name in san.iter() {
                if let Some(dns) = name.dnsname() {
                    hostnames.push(dns.to_string());
                }
            }
        }

        if hostnames.is_empty() {
            let subject = self.certificate.subject_name();
            for entry in subject.entries_by_nid(pingora_openssl::nid::Nid::COMMONNAME) {
                if let Ok(cn) = entry.data().as_utf8() {
                    hostnames.push(cn.to_string());
                }
            }
        }

        hostnames
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use openssl::asn1::Asn1Time;
    use openssl::hash::MessageDigest;
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;
    use openssl::x509::extension::SubjectAlternativeName;
    use openssl::x509::{X509Builder, X509NameBuilder};

    fn generate_test_cert(days_valid: u32, days_offset: i64) -> (String, String) {
        let rsa = Rsa::generate(2048).unwrap();
        let key = PKey::from_rsa(rsa).unwrap();

        let mut name = X509NameBuilder::new().unwrap();
        name.append_entry_by_text("CN", "test.example.com").unwrap();
        let name = name.build();

        let mut builder = X509Builder::new().unwrap();
        builder.set_subject_name(&name).unwrap();
        builder.set_issuer_name(&name).unwrap();
        builder.set_pubkey(&key).unwrap();
        builder
            .set_not_before(&Asn1Time::days_from_now(days_offset.max(0) as u32).unwrap())
            .unwrap();
        builder
            .set_not_after(&Asn1Time::days_from_now(days_valid).unwrap())
            .unwrap();
        builder.sign(&key, MessageDigest::sha256()).unwrap();

        let cert = builder.build();
        let cert_pem = String::from_utf8(cert.to_pem().unwrap()).unwrap();
        let key_pem = String::from_utf8(key.private_key_to_pem_pkcs8().unwrap()).unwrap();

        (cert_pem, key_pem)
    }

    fn generate_cert_with_san(cn: &str, sans: &[&str]) -> (String, String) {
        let rsa = Rsa::generate(2048).unwrap();
        let key = PKey::from_rsa(rsa).unwrap();

        let mut name = X509NameBuilder::new().unwrap();
        name.append_entry_by_text("CN", cn).unwrap();
        let name = name.build();

        let mut builder = X509Builder::new().unwrap();
        builder.set_subject_name(&name).unwrap();
        builder.set_issuer_name(&name).unwrap();
        builder.set_pubkey(&key).unwrap();
        builder
            .set_not_before(&Asn1Time::days_from_now(0).unwrap())
            .unwrap();
        builder
            .set_not_after(&Asn1Time::days_from_now(365).unwrap())
            .unwrap();

        if !sans.is_empty() {
            let context = builder.x509v3_context(None, None);
            let mut san_ext = SubjectAlternativeName::new();
            for san in sans {
                san_ext.dns(san);
            }
            let ext = san_ext.build(&context).unwrap();
            builder.append_extension(ext).unwrap();
        }

        builder.sign(&key, MessageDigest::sha256()).unwrap();
        let cert = builder.build();

        let cert_pem = String::from_utf8(cert.to_pem().unwrap()).unwrap();
        let key_pem = String::from_utf8(key.private_key_to_pem_pkcs8().unwrap()).unwrap();

        (cert_pem, key_pem)
    }

    #[test]
    fn test_new_valid_cert() {
        let (cert_pem, key_pem) = generate_test_cert(365, 0);
        let result = Certificate::new(&cert_pem, &key_pem);
        assert!(result.is_ok(), "Should construct with valid cert+key");
    }

    #[test]
    fn test_new_mismatched_key() {
        let (cert_pem, _) = generate_test_cert(365, 0);
        let (_, other_key_pem) = generate_test_cert(365, 0);

        let result = Certificate::new(&cert_pem, &other_key_pem);
        assert!(result.is_err(), "Should fail with mismatched cert+key");
    }

    #[test]
    fn test_new_invalid_pem() {
        let result = Certificate::new("not a cert", "not a key");
        assert!(result.is_err(), "Should fail with invalid PEM");
    }

    #[test]
    fn test_expires_at_is_in_future() {
        let (cert_pem, key_pem) = generate_test_cert(365, 0);
        let cert = Certificate::new(&cert_pem, &key_pem).unwrap();

        let expires = cert.expires_at().unwrap();
        assert!(expires > Utc::now(), "Expiry should be in the future");
    }

    #[test]
    fn test_expires_at_roughly_correct() {
        let (cert_pem, key_pem) = generate_test_cert(365, 0);
        let cert = Certificate::new(&cert_pem, &key_pem).unwrap();

        let expires = cert.expires_at().unwrap();
        let days_until_expiry = (expires - Utc::now()).num_days();

        assert!(
            days_until_expiry >= 364 && days_until_expiry <= 366,
            "Expiry should be ~365 days from now, got {} days",
            days_until_expiry
        );
    }

    #[test]
    fn test_expires_at_short_lived_cert() {
        let (cert_pem, key_pem) = generate_test_cert(1, 0);
        let cert = Certificate::new(&cert_pem, &key_pem).unwrap();

        let expires = cert.expires_at().unwrap();
        let days_until_expiry = (expires - Utc::now()).num_days();

        assert!(
            days_until_expiry == 0 || days_until_expiry == 1,
            "Short-lived cert should expire in ~1 day"
        );
    }

    #[test]
    fn test_hostnames_single_san() {
        let (cert_pem, key_pem) = generate_cert_with_san("example.com", &["example.com"]);
        let cert = Certificate::new(&cert_pem, &key_pem).unwrap();

        assert_eq!(cert.hostnames(), vec!["example.com"]);
    }

    #[test]
    fn test_hostnames_multiple_sans() {
        let (cert_pem, key_pem) = generate_cert_with_san(
            "example.com",
            &["example.com", "www.example.com", "api.example.com"],
        );
        let cert = Certificate::new(&cert_pem, &key_pem).unwrap();
        let hostnames = cert.hostnames();

        assert_eq!(hostnames.len(), 3);
        assert!(hostnames.contains(&"example.com".to_string()));
        assert!(hostnames.contains(&"www.example.com".to_string()));
        assert!(hostnames.contains(&"api.example.com".to_string()));
    }

    #[test]
    fn test_hostnames_wildcard_san() {
        let (cert_pem, key_pem) =
            generate_cert_with_san("example.com", &["*.example.com", "example.com"]);
        let cert = Certificate::new(&cert_pem, &key_pem).unwrap();
        let hostnames = cert.hostnames();

        assert!(hostnames.contains(&"*.example.com".to_string()));
        assert!(hostnames.contains(&"example.com".to_string()));
    }

    #[test]
    fn test_hostnames_fallback_to_cn() {
        let (cert_pem, key_pem) = generate_cert_with_san("example.com", &[]);
        let cert = Certificate::new(&cert_pem, &key_pem).unwrap();

        assert_eq!(cert.hostnames(), vec!["example.com"]);
    }

    #[test]
    fn test_hostnames_san_takes_priority_over_cn() {
        let (cert_pem, key_pem) = generate_cert_with_san("example.com", &["www.example.com"]);
        let cert = Certificate::new(&cert_pem, &key_pem).unwrap();
        let hostnames = cert.hostnames();

        assert_eq!(hostnames, vec!["www.example.com"]);
        assert!(!hostnames.contains(&"example.com".to_string()));
    }
}
