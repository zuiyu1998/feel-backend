use abi::{
    thiserror::{self, Error},
    Error as AbiError,
};
use std::{error::Error as StdError, io::Error as IoError};
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
    #[error("internal server error: {0}")]
    InternalServer(String),
    #[error("request error: {0}")]
    RequestError(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for poem::Error {
    fn from(value: Error) -> Self {
        let error: Box<dyn StdError + Send + Sync> = Box::new(value);

        poem::Error::from(error)
    }
}
