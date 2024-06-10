use abi::{
    thiserror::{self, Error},
    Error as AbiError,
};
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
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
