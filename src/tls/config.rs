use std::env;
use std::io;

#[derive(Debug)]
pub struct PKeyEncryptionConfig {
    pub(crate) key: [u8; 32],
}

impl PKeyEncryptionConfig {
    pub fn from_env() -> Result<Self, io::Error> {
        let raw = env::var("CERT_ENCRYPTION_KEY")
            .map_err(|e| io::Error::other(format!("CERT_ENCRYPTION_KEY: {}", e)))?;

        let key: [u8; 32] = if raw.len() == 64 {
            decode_hex(&raw)
                .map_err(|e| io::Error::other(format!("CERT_ENCRYPTION_KEY: invalid hex: {}", e)))?
                .try_into()
                .map_err(|_| {
                    io::Error::other("CERT_ENCRYPTION_KEY: hex must decode to exactly 32 bytes")
                })?
        } else if raw.len() == 32 {
            raw.as_bytes().try_into().map_err(|_| {
                io::Error::other("CERT_ENCRYPTION_KEY: failed to read as 32-byte key")
            })?
        } else {
            return Err(io::Error::other(format!(
                "CERT_ENCRYPTION_KEY: expected 32 raw chars or 64 hex chars, got {} chars",
                raw.len()
            )));
        };

        Ok(Self { key })
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, io::Error> {
    if s.len() % 2 != 0 {
        return Err(io::Error::other("hex string must have even length"));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|e| io::Error::other(format!("invalid hex char at position {}: {}", i, e)))
        })
        .collect()
}
