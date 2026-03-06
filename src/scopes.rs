use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserScope {
    ProxyHostsRead,
    ProxyHostsWrite,
    ProxyHostsDelete,

    RedirectionHostsRead,
    RedirectionHostsWrite,
    RedirectionHostsDelete,

    CertificatesRead,
    CertificatesWrite,
    CertificatesDelete,
}

impl fmt::Display for UserScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::ProxyHostsRead => "proxy_hosts_read",
            Self::ProxyHostsWrite => "proxy_hosts_write",
            Self::ProxyHostsDelete => "proxy_hosts_delete",

            Self::RedirectionHostsRead => "redirection_hosts_read",
            Self::RedirectionHostsWrite => "redirection_hosts_write",
            Self::RedirectionHostsDelete => "redirection_hosts_delete",

            Self::CertificatesRead => "certificates_read",
            Self::CertificatesWrite => "certificates_write",
            Self::CertificatesDelete => "certificates_delete",
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
            "proxy_hosts_delete" => Ok(Self::ProxyHostsDelete),

            "redirection_hosts_read" => Ok(Self::RedirectionHostsRead),
            "redirection_hosts_write" => Ok(Self::RedirectionHostsWrite),
            "redirection_hosts_delete" => Ok(Self::RedirectionHostsDelete),

            "certificates_read" => Ok(Self::CertificatesRead),
            "certificates_write" => Ok(Self::CertificatesWrite),
            "certificates_delete" => Ok(Self::CertificatesDelete),

            other => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected scope value: {other}"),
            )),
        }
    }
}
