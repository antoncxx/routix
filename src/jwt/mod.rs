use jsonwebtoken::{Header, TokenData, Validation, decode, encode};

mod claims;
mod config;
mod error;

pub use claims::*;
pub use config::*;
pub use error::*;

#[derive(Debug)]
pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn issue(&self, subject: &str, roles: Vec<String>) -> Result<String, JwtError> {
        let now = current_timestamp();
        let claims = Claims {
            sub: subject.to_owned(),
            iat: now as usize,
            exp: (now + self.config.expiry_seconds) as usize,
            roles,
        };
        Ok(encode(
            &Header::default(),
            &claims,
            &self.config.encoding_key,
        )?)
    }

    pub fn verify(&self, token: &str) -> Result<TokenData<Claims>, JwtError> {
        let validation = Validation::default();
        decode::<Claims>(token, &self.config.decoding_key, &validation).map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::Expired,
                _ => JwtError::VerifyError(e),
            }
        })
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_service() -> JwtService {
        JwtService::new(JwtConfig::from_secret(b"super-secret-key", 3600))
    }

    #[test]
    fn test_issue_and_verify() {
        let svc = test_service();
        let token = svc.issue("user-123", vec!["admin".into()]).unwrap();
        let data = svc.verify(&token).unwrap();

        assert_eq!(data.claims.sub, "user-123");
        assert!(data.claims.roles.contains(&"admin".to_owned()));
    }

    #[test]
    fn test_expired_token() {
        use jsonwebtoken::{Header, encode};

        let svc = test_service();

        let claims = Claims {
            sub: "user-123".to_owned(),
            iat: 0,
            exp: 1,
            roles: vec![],
        };

        let token = encode(&Header::default(), &claims, &svc.config.encoding_key).unwrap();
        let err = svc.verify(&token).unwrap_err();
        assert!(matches!(err, JwtError::Expired));
    }

    #[test]
    fn test_invalid_token() {
        let svc = test_service();
        let err = svc.verify("not.a.token").unwrap_err();
        assert!(matches!(err, JwtError::VerifyError(_)));
    }
}
