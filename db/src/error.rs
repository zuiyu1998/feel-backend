use abi::{
    sea_orm::DbErr,
    thiserror::{self, Error},
    tonic::{transport::Error as TonicTransportError, Status},
    Error as AbiError,
};
use std::io::Error as IoError;
use utils::Error as UtilsError;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("UserNotFound")]
    UserNotFound,
    #[error("AuthNotFound")]
    AuthNotFound,
    #[error("PasswordError")]
    PasswordError,
}

#[derive(Debug, Error)]

pub enum Error {
    #[error("Kind: {0}")]
    Kind(#[from] Kind),
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
    #[error("db error: {0}")]
    DbErr(#[from] DbErr),
    #[error("utils error: {0}")]
    UtilsError(#[from] UtilsError),
    #[error("tonic transport error: {0}")]
    TonicTransportError(#[from] TonicTransportError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        Status::internal(value.to_string())
    }
}
