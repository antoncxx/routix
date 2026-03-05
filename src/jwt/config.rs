use super::error::JwtError;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::env;

#[derive(Debug)]
pub struct JwtConfig {
    pub(crate) encoding_key: EncodingKey,
    pub(crate) decoding_key: DecodingKey,
    pub(crate) expiry_seconds: u64,
}

const DEFAULT_EXPIRY_SECONDS: u64 = 3600;

impl JwtConfig {
    pub fn from_env() -> Result<Self, JwtError> {
        let secret = Self::read_secret()?;

        let expiry_seconds = env::var("JWT_EXPIRY_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_EXPIRY_SECONDS);

        Ok(Self::from_secret(secret.as_bytes(), expiry_seconds))
    }

    pub fn from_secret(secret: &[u8], expiry_seconds: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            expiry_seconds,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn read_secret() -> Result<String, JwtError> {
        #[cfg(debug_assertions)]
        {
            const DEFAULT_DEBUG_SECRET: &str = "debug-secret";

            Ok(env::var("JWT_SECRET").unwrap_or_else(|_| {
                log::warn!("WARNING: using default JWT secret, do not use in production");
                DEFAULT_DEBUG_SECRET.to_owned()
            }))
        }

        #[cfg(not(debug_assertions))]
        {
            env::var("JWT_SECRET").map_err(|_| JwtError::MissingSecret)
        }
    }
}
