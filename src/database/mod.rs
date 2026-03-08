use deadpool_diesel::postgres::{Connection, Manager, Pool};
use deadpool_diesel::{PoolError, Runtime};
use std::error::Error;

pub mod config;
pub mod models;
pub mod repos;
pub mod schema;

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(config: config::DatabaseConfig) -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new(config.database_url, Runtime::Tokio1);

        let pool = Pool::builder(manager)
            .max_size(config.max_connections)
            .build()?;

        Ok(Self { pool })
    }

    pub async fn connection(&self) -> Result<Connection, PoolError> {
        self.pool.get().await
    }
}
