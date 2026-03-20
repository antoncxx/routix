use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserScope {
    ProxyHostsRead,
    ProxyHostsWrite,

    UpstreamsRead,
    UpstreamsWrite,

    CertificatesRead,
    CertificatesWrite,

    AccessListsRead,
    AccessListsWrite,
}

impl fmt::Display for UserScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::ProxyHostsRead => "proxy_hosts_read",
            Self::ProxyHostsWrite => "proxy_hosts_write",

            Self::UpstreamsRead => "upstreams_read",
            Self::UpstreamsWrite => "upstreams_write",

            Self::CertificatesRead => "certificates_read",
            Self::CertificatesWrite => "certificates_write",

            Self::AccessListsRead => "access_lists_read",
            Self::AccessListsWrite => "access_lists_write",
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

            "upstreams_read" => Ok(Self::UpstreamsRead),
            "upstreams_write" => Ok(Self::UpstreamsWrite),

            "certificates_read" => Ok(Self::CertificatesRead),
            "certificates_write" => Ok(Self::CertificatesWrite),

            "access_lists_read" => Ok(Self::AccessListsRead),
            "access_lists_write" => Ok(Self::AccessListsWrite),

            other => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected scope value: {other}"),
            )),
        }
    }
}
