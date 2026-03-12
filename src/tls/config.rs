use anyhow::{Context, anyhow};
use std::env;

#[derive(Debug)]
pub struct PKeyEncryptionConfig {
    pub(crate) key: [u8; 32],
}

impl PKeyEncryptionConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let raw = env::var("CERT_ENCRYPTION_KEY").context("CERT_ENCRYPTION_KEY not set")?;

        let key: [u8; 32] = if raw.len() == 64 {
            decode_hex(&raw)
                .context("CERT_ENCRYPTION_KEY: invalid hex")?
                .try_into()
                .map_err(|_| anyhow!("CERT_ENCRYPTION_KEY: hex must decode to exactly 32 bytes"))?
        } else if raw.len() == 32 {
            raw.as_bytes()
                .try_into()
                .map_err(|_| anyhow!("CERT_ENCRYPTION_KEY: failed to read as 32-byte key"))?
        } else {
            return Err(anyhow!(
                "CERT_ENCRYPTION_KEY: expected 32 raw chars or 64 hex chars, got {} chars",
                raw.len()
            ));
        };

        Ok(Self { key })
    }
}

fn decode_hex(s: &str) -> anyhow::Result<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return Err(anyhow!("hex string must have even length"));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .with_context(|| format!("invalid hex char at position {i}"))
        })
        .collect()
}
