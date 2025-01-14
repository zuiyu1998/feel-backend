use std::io::Error as IoError;
use thiserror::Error;
use sea_orm::DbErr;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("db error: {0}")]
    DbErr(#[from] DbErr),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
