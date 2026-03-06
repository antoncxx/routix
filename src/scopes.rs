use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserScope {
    UsersRead,
    UsersWrite,
    UsersDelete,
}

impl fmt::Display for UserScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::UsersRead => "users_read",
            Self::UsersWrite => "users_write",
            Self::UsersDelete => "users_delete",
        };
        write!(f, "{s}")
    }
}

impl FromStr for UserScope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "users_read" => Ok(Self::UsersRead),
            "users_write" => Ok(Self::UsersWrite),
            "users_delete" => Ok(Self::UsersDelete),
            other => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected scope value: {other}"),
            )),
        }
    }
}
