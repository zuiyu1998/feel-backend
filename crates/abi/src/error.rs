use sea_orm::DbErr;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("PasswordNotMatch")]
    PasswordNotMatch,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("db error: {0}")]
    DbErr(#[from] DbErr),
    #[error("kind: {0}")]
    Kind(#[from] ErrorKind),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
