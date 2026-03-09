use pingora::prelude::HttpPeer;

use crate::database::models::ProxyHostModel;

#[derive(Debug, Clone)]
pub enum ProxyHostSchema {
    Http,
    Https,
}

impl TryFrom<&str> for ProxyHostSchema {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            _ => Err(format!("Invalid proxy schema: {value}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProxyHost {
    pub domain: String,
    pub forward_host: String,
    pub forward_port: i32,
    pub forward_schema: ProxyHostSchema,
    pub certificate_name: Option<String>,
}

impl TryFrom<ProxyHostModel> for ProxyHost {
    type Error = String;

    fn try_from(value: ProxyHostModel) -> Result<Self, Self::Error> {
        let forward_schema = ProxyHostSchema::try_from(value.forward_schema.as_str())?;

        Ok(Self {
            domain: value.domain,
            forward_host: value.forward_host,
            forward_port: value.forward_port,
            forward_schema,
            certificate_name: value.certificate_name,
        })
    }
}

impl ProxyHost {
    pub fn is_https(&self) -> bool {
        matches!(self.forward_schema, ProxyHostSchema::Https)
    }

    pub fn upstream(&self) -> Box<HttpPeer> {
        let address = format!("{}:{}", self.forward_host, self.forward_port);
        let peer = HttpPeer::new(address, self.is_https(), self.forward_host.clone());
        Box::new(peer)
    }

    pub fn upstream_host_header(&self) -> String {
        match (&self.forward_schema, self.forward_port) {
            (ProxyHostSchema::Http, 80) | (ProxyHostSchema::Https, 443) => {
                self.forward_host.clone()
            }
            (_, port) => format!("{}:{}", self.forward_host, port),
        }
    }
}
