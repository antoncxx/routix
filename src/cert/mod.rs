mod dns_providers;
use anyhow::Ok;
use async_trait::async_trait;
pub use dns_providers::*;
use std::fmt;

mod authority;

pub use authority::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsProviderId {
    Cloudflare,
}

impl fmt::Display for DnsProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DnsProviderId::Cloudflare => write!(f, "cloudflare"),
        }
    }
}

impl std::str::FromStr for DnsProviderId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cloudflare" => Ok(DnsProviderId::Cloudflare),
            _ => anyhow::bail!("Unknown DNS provider: '{s}'."),
        }
    }
}

#[async_trait]
pub trait DnsProvider: Send + Sync {
    async fn create_txt_record(
        &self,
        name: &str,  // e.g. "_acme-challenge.example.com"
        value: &str, // the ACME key authorization digest
    ) -> anyhow::Result<String>;

    async fn delete_txt_record(&self, record_id: &str) -> anyhow::Result<()>;
}

pub fn create_dns_provider(provider_id: DnsProviderId) -> anyhow::Result<Box<dyn DnsProvider>> {
    let provider = match provider_id {
        DnsProviderId::Cloudflare => {
            let config = CloudflareDnsConfig::from_env()?;
            CloudflareDns::new(config)
        }
    };

    Ok(Box::new(provider))
}
