pub mod sha2_impl;

pub trait Encryptor: 'static + Send + Sync {
    fn encode(&self, bytes: &[u8]) -> Vec<u8>;
}

