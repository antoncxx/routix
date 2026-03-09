use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use base64::{Engine, engine::general_purpose::STANDARD};

pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key);
        Self {
            cipher: Aes256Gcm::new(key),
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let mut combined = nonce_bytes.to_vec();

        combined.extend(
            self.cipher
                .encrypt(nonce, plaintext.as_bytes())
                .map_err(|e| format!("Encryptor::encrypt failed: {e}"))?,
        );

        Ok(STANDARD.encode(combined))
    }

    pub fn decrypt(&self, encoded: &str) -> Result<String, String> {
        let combined = STANDARD
            .decode(encoded)
            .map_err(|e| format!("Encryptor::decrypt: invalid base64: {e}"))?;

        if combined.len() < 12 {
            return Err("Encryptor::decrypt: data too short to contain nonce".into());
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Encryptor::decrypt: authentication failed: {e}"))?;

        String::from_utf8(plaintext).map_err(|e| format!("Encryptor::decrypt: invalid utf8: {e}"))
    }
}
