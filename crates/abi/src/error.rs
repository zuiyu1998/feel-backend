use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
