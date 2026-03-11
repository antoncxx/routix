use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct CloudflareDnsConfig {
    pub api_token: String,
}

impl CloudflareDnsConfig {
    pub fn from_env() -> Result<Self> {
        let api_token = env::var("CF_API_TOKEN")
            .context("Missing required environment variable: CF_API_TOKEN")?;

        Ok(Self { api_token })
    }
}
