use config::ConfigError;
use redis::RedisError;
use sea_orm::DbErr;
use serde_json::Error as SerdeJsonError;
use std::io::Error as IoError;
use thiserror::Error;

use protocol::tonic::Status;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("PasswordNotMatch")]
    PasswordNotMatch,
    #[error("TokenInvalid")]
    TokenInvalid,
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
    #[error("redis error: {0}")]
    RedisError(#[from] RedisError),
    #[error("serde json error: {0}")]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("config error: {0}")]
    ConfigError(#[from] ConfigError),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        Status::internal(value.to_string())
    }
}
