use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;

pub(crate) static DOMAIN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap()
});

pub(crate) static HOST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9-]+$").unwrap()
});

pub(crate) fn validate_forward_schema(schema: &str) -> Result<(), validator::ValidationError> {
    match schema {
        "http" | "https" => Ok(()),
        _ => Err(validator::ValidationError::new(
            "forward_schema must be 'http' or 'https'",
        )),
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
