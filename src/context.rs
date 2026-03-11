use std::{error::Error, sync::Arc};

use crate::{
    cert::CertificateAuthority,
    database::{Database, config::DatabaseConfig},
    jwt::{JwtConfig, JwtService},
    proxy::HostsManager,
    tls::{CertificatesManager, PKeyEncryptionConfig},
};

#[derive(Clone)]
pub struct Context {
    pub jwt: Arc<JwtService>,
    pub database: Arc<Database>,
    pub certificates_manager: Arc<CertificatesManager>,
    pub hosts_manager: Arc<HostsManager>,
    pub certificate_authority: Arc<CertificateAuthority>,
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
            CertificatesManager::new(&config).into()
        };

        log::debug!("Creating hosts manager");
        let hosts_manager = Arc::new(HostsManager::new());

        log::debug!("Creating certificates authority");
        let certificate_authority = if cfg!(debug_assertions) {
            CertificateAuthority::staging().into()
        } else {
            CertificateAuthority::production().into()
        };

        Ok(Self {
            jwt,
            database,
            certificates_manager,
            hosts_manager,
            certificate_authority,
        })
    }
}
