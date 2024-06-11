use abi::{
    thiserror::{self, Error},
    Error as AbiError,
};
use jsonwebtoken::errors::Error as JsonwebtokenError;
use std::{error::Error as StdError, io::Error as IoError};
use utils::Error as UtilsError;

use crate::apis::utils::*;
use poem::{Body, Response};

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
    #[error("jwt error: {0}")]
    JsonwebtokenError(#[from] JsonwebtokenError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for poem::Error {
    fn from(value: Error) -> Self {
        let error: Box<dyn StdError + Send + Sync> = Box::new(value);

        poem::Error::from(error)
    }
}

pub async fn handle_error(e: Error) -> Response {
    let msg = e.to_string();

    let builder = Response::builder().content_type("application/json; charset=utf-8");

    let mut body = ResponseObject::<String>::error(&msg);

    match e {
        Error::RequestError(_) => body.code = ResponseCode::InvalidRequest as i32,
        _ => {}
    }

    let body = Body::from_json(body).unwrap();

    let res = builder.body(body);

    res
}
