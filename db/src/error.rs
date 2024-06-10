use abi::{
    thiserror::{self, Error},
    Error as AbiError,
};
use entity::sea_orm::DbErr;
use std::io::Error as IoError;

#[derive(Debug, Error)]
pub enum Kind {}

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
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
