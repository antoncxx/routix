use ipnet::IpNet;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::sync::LazyLock;
use validator::Validate;

pub(crate) static DOMAIN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap()
});

pub(crate) static HOST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9-]+$").unwrap()
});

pub(crate) static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap());

pub(crate) fn validate_forward_schema(schema: &str) -> Result<(), validator::ValidationError> {
    match schema {
        "http" | "https" => Ok(()),
        _ => Err(validator::ValidationError::new(
            "forward_schema must be 'http' or 'https'",
        )),
    }
}

pub(crate) fn validate_access_list_action(action: &str) -> Result<(), validator::ValidationError> {
    match action {
        "allow" | "deny" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_action")),
    }
}

pub(crate) fn validate_address(address: &str) -> Result<(), validator::ValidationError> {
    let valid = address.parse::<IpAddr>().is_ok() || address.parse::<IpNet>().is_ok();

    if valid {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_address"))
    }
}
#[derive(Serialize)]
pub(crate) struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Deserialize, Validate)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    pub(crate) page: i64,
    #[serde(default = "default_per_page")]
    #[validate(range(min = 1, max = 100))]
    pub(crate) per_page: i64,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    20
}
