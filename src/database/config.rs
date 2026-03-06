use std::error::Error;

#[derive(Debug)]
pub struct DatabaseConfig {
    pub(crate) database_url: String,
    pub(crate) max_connections: usize,
}

const DEFAULT_MAX_CONNECTIONS: usize = 10;

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL environment variable not set")?;

        let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| DEFAULT_MAX_CONNECTIONS.to_string())
            .parse::<usize>()
            .unwrap_or_else(|_| {
                log::warn!("DATABASE_MAX_CONNECTIONS is not a valid number, defaulting to {DEFAULT_MAX_CONNECTIONS}");
                DEFAULT_MAX_CONNECTIONS
            });

        Ok(Self {
            database_url,
            max_connections,
        })
    }
}
