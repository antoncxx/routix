use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Admin => "admin",
            Self::User => "user",
        };
        write!(f, "{s}")
    }
}

impl FromStr for UserRole {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Self::Admin),
            "user" => Ok(Self::User),
            other => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected user role: {other}"),
            )),
        }
    }
}
