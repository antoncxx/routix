use std::{collections::HashMap, sync::Arc};

pub use certificate::Certificate;
pub use config::PKeyEncryptionConfig;
use encryptor::Encryptor;
use tokio::sync::RwLock;

mod certificate;
mod config;
mod encryptor;

pub struct CertificatesManager {
    certs: RwLock<HashMap<String, Arc<Certificate>>>,
    encryptor: Encryptor,
}

impl CertificatesManager {
    pub fn new(config: PKeyEncryptionConfig) -> Self {
        let encryptor = Encryptor::new(&config.key);
        let certs = Default::default();
        Self { encryptor, certs }
    }

    pub fn encrypt_certificate_key(&self, cert_key: &str) -> Result<String, String> {
        self.encryptor.encrypt(cert_key)
    }

    pub fn decrypt_certificate_key(&self, cert_key: &str) -> Result<String, String> {
        self.encryptor.decrypt(cert_key)
    }

    pub async fn add(&self, name: &str, certificate: Certificate) -> bool {
        let mut map = self.certs.write().await;

        if map.contains_key(name) {
            return false;
        }

        map.insert(name.to_string(), Arc::new(certificate));

        true
    }

    pub async fn remove(&self, name: &str) -> bool {
        self.certs.write().await.remove(name).is_some()
    }

    pub async fn get(&self, name: &str) -> Option<Arc<Certificate>> {
        self.certs.read().await.get(name).cloned()
    }
}
