use sha2::{Digest, Sha256};

use abi::{
    config::{Config, FromConfig},
    tonic::async_trait,
};

use crate::Error;

pub struct ShaHelper {
    secure: String,
}

impl ShaHelper {
    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();

        hasher.update(self.secure.as_bytes());
        hasher.update(data);

        let result = hasher.finalize().to_vec();

        result
    }
}

#[async_trait]
impl FromConfig for ShaHelper {
    type Error = Error;

    async fn from_config(config: &Config) -> Result<Self, Self::Error> {
        Ok(ShaHelper {
            secure: config.db.secure.to_string(),
        })
    }
}
