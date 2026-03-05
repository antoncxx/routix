use std::{error::Error, sync::Arc};

use crate::jwt::{JwtConfig, JwtService};

#[derive(Debug, Clone)]
pub struct Context {
    pub jwt: Arc<JwtService>,
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let jwt = {
            let config = JwtConfig::from_env()?;
            JwtService::new(config).into()
        };

        Ok(Self { jwt })
    }
}
