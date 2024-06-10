use figment::Error as FigmentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {}

#[derive(Debug, Error)]

pub enum Error {
    #[error("Kind: {0}")]
    Kind(#[from] Kind),
    #[error("figment error: {0}")]
    FigmentError(#[from] FigmentError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
