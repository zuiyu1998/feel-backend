use sea_orm::DbErr;
use std::io::Error as IoError;
use thiserror::Error;

use protocol::tonic::Status;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("PasswordNotMatch")]
    PasswordNotMatch,
    #[error("TokenInvaild")]
    TokenInvaild,
    #[error("AuthNotFound")]
    AuthNotFound,
    #[error("UserNotFound")]
    UserNotFound,
    #[error("ParseError")]
    ParseError,
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

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        Status::internal(value.to_string())
    }
}
