use anyhow::{Result, anyhow};
use pingora::prelude::HttpPeer;

use crate::database::models::UpstreamModel;

#[derive(Debug, Clone)]
pub enum UpstreamSchema {
    Http,
    Https,
}

impl TryFrom<&str> for UpstreamSchema {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            _ => Err(anyhow!("Invalid upstream schema: {value}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Upstream {
    pub _name: String,
    pub host: String,
    pub port: i32,
    pub schema: UpstreamSchema,
}

impl TryFrom<UpstreamModel> for Upstream {
    type Error = anyhow::Error;

    fn try_from(value: UpstreamModel) -> Result<Self> {
        let schema = UpstreamSchema::try_from(value.schema.as_str())?;

        Ok(Self {
            _name: value.name,
            host: value.host,
            port: value.port,
            schema,
        })
    }
}

impl Upstream {
    pub fn is_https(&self) -> bool {
        matches!(self.schema, UpstreamSchema::Https)
    }

    pub fn to_peer(&self) -> Box<HttpPeer> {
        let address = format!("{}:{}", self.host, self.port);
        let peer = HttpPeer::new(address, self.is_https(), self.host.clone());
        Box::new(peer)
    }

    pub fn host_header(&self) -> String {
        match (&self.schema, self.port) {
            (UpstreamSchema::Http, 80) | (UpstreamSchema::Https, 443) => self.host.clone(),
            (_, port) => format!("{}:{}", self.host, port),
        }
    }
}
