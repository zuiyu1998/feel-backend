use abi::thiserror::{self, Error};
use std::io::Error as IoError;

#[derive(Debug, Error)]
pub enum Kind {}

#[derive(Debug, Error)]

pub enum Error {
    #[error("Kind: {0}")]
    Kind(#[from] Kind),
    #[error("io error: {0}")]
    IoError(#[from] IoError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
