pub use certificate::Certificate;
pub use config::PKeyEncryptionConfig;
use encryptor::Encryptor;

mod certificate;
mod config;
mod encryptor;

pub struct CertificatesManager {
    // ????????????????????????????????????????????????
    // certs: RwLock<HashMap<String, Arc<Certificate>>>,
    encryptor: Encryptor,
}

impl CertificatesManager {
    pub fn new(config: PKeyEncryptionConfig) -> Self {
        let encryptor = Encryptor::new(&config.key);
        Self { encryptor }
    }

    pub fn encrypt_certificate_key(&self, cert_key: &str) -> Result<String, String> {
        self.encryptor.encrypt(cert_key)
    }

    pub fn decrypt_certificate_key(&self, cert_key: &str) -> Result<String, String> {
        self.encryptor.decrypt(cert_key)
    }

    // pub async fn add(&self, domain: &str, certificate: Certificate) -> bool {
    //     let mut map = self.certs.write().await;

    //     if map.contains_key(domain) {
    //         return false;
    //     }

    //     map.insert(domain.to_string(), Arc::new(certificate));

    //     true
    // }

    // pub async fn remove(&self, domain: &str) -> bool {
    //     self.certs.write().await.remove(domain).is_some()
    // }

    // pub async fn get(&self, domain: &str) -> Option<Arc<Certificate>> {
    //     self.certs.read().await.get(domain).cloned()
    // }
}
