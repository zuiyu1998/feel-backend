use super::Encryptor;
use abi::config::ShaConfig;
use sha2::{Digest, Sha256};

pub struct Sha2Encryptor {
    security: String,
}

impl Sha2Encryptor {
    pub fn from_config(config: &ShaConfig) -> Self {
        Sha2Encryptor { security: config.security.clone() }
    }
}

impl Encryptor for Sha2Encryptor {
    fn encode(&self, bytes: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.security.as_bytes());
        hasher.update(bytes);

        let result = hasher.finalize();

        result.to_vec()
    }
}
