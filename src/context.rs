use std::{error::Error, sync::Arc};

use crate::{
    database::{Database, config::DatabaseConfig},
    jwt::{JwtConfig, JwtService},
    tls::{CertificatesManager, PKeyEncryptionConfig},
};

#[derive(Clone)]
pub struct Context {
    pub jwt: Arc<JwtService>,
    pub database: Arc<Database>,
    pub certificates_manager: Arc<CertificatesManager>,
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        log::debug!("Initializing JWT service");

        let jwt = {
            let config = JwtConfig::from_env()?;
            JwtService::new(config).into()
        };

        log::debug!("Initializing Database connection");

        let database = {
            let config = DatabaseConfig::from_env()?;
            Database::new(config)?.into()
        };

        log::debug!("Creating certificates manager");

        let certificates_manager = {
            let config = PKeyEncryptionConfig::from_env()?;
            CertificatesManager::new(config).into()
        };

        Ok(Self {
            jwt,
            database,
            certificates_manager,
        })
    }
}
