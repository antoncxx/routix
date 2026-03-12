use regex::Regex;
use std::sync::LazyLock;

pub(super) static DOMAIN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap()
});

pub(super) static HOST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9-]+$").unwrap()
});

pub(super) fn validate_forward_schema(schema: &str) -> Result<(), validator::ValidationError> {
    match schema {
        "http" | "https" => Ok(()),
        _ => Err(validator::ValidationError::new(
            "forward_schema must be 'http' or 'https'",
        )),
    }
}
