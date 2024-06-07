use abi::thiserror::Error;
use std::io::Error as IoError;
use utils::Error as UtilsError;

#[derive(Debug, Error)]
pub enum Kind {}

#[derive(Debug, Error)]

pub enum Error {
    #[error("Kind: {0}")]
    Kind(#[from] Kind),
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("utils error: {0}")]
    UtilsError(#[from] UtilsError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
