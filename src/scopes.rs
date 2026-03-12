use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserScope {
    ProxyHostsRead,
    ProxyHostsWrite,

    RedirectionHostsRead,
    RedirectionHostsWrite,

    CertificatesRead,
    CertificatesWrite,
}

impl fmt::Display for UserScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::ProxyHostsRead => "proxy_hosts_read",
            Self::ProxyHostsWrite => "proxy_hosts_write",

            Self::RedirectionHostsRead => "redirection_hosts_read",
            Self::RedirectionHostsWrite => "redirection_hosts_write",

            Self::CertificatesRead => "certificates_read",
            Self::CertificatesWrite => "certificates_write",
        };
        write!(f, "{s}")
    }
}

impl FromStr for UserScope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "proxy_hosts_read" => Ok(Self::ProxyHostsRead),
            "proxy_hosts_write" => Ok(Self::ProxyHostsWrite),

            "redirection_hosts_read" => Ok(Self::RedirectionHostsRead),
            "redirection_hosts_write" => Ok(Self::RedirectionHostsWrite),

            "certificates_read" => Ok(Self::CertificatesRead),
            "certificates_write" => Ok(Self::CertificatesWrite),

            other => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected scope value: {other}"),
            )),
        }
    }
}
