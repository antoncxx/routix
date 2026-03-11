use anyhow::{Context, Result};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct ApiConfig {
    pub(crate) addr: SocketAddr,
}

const DEFAULT_API_HOST: &str = "0.0.0.0";
const DEFAULT_API_PORT: u16 = 8181;

impl ApiConfig {
    pub fn from_env() -> Result<Self> {
        let host = std::env::var("API_HOST").unwrap_or_else(|_| DEFAULT_API_HOST.to_owned());
        let port = std::env::var("API_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(DEFAULT_API_PORT);

        let addr = format!("{host}:{port}")
            .parse::<SocketAddr>()
            .with_context(|| format!("Invalid socket address: {host}:{port}"))?;

        Ok(Self { addr })
    }
}
