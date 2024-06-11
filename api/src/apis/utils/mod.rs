mod auth;

pub use auth::*;

use poem_openapi::{
    error::ParseRequestPayloadError,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};

use serde::{Deserialize, Serialize};

use poem::Error;

pub enum ResponseCode {
    Success = 10000,
    Unknown = -10000,
    InvalidRequest = -2,
    InternalServerError = 500,
}

#[derive(Deserialize, Serialize, Object)]
pub struct Empty;

#[derive(Object, Serialize, Deserialize)]
pub struct ResponseObject<T: ParseFromJSON + ToJSON + Send + Sync> {
    pub code: i32,
    msg: String,
    data: Option<T>,
}

impl<T: ParseFromJSON + ToJSON> ResponseObject<T> {
    pub fn error(message: &str) -> Self {
        ResponseObject {
            code: ResponseCode::InternalServerError as i32,
            msg: message.to_string(),
            data: None,
        }
    }
}

#[derive(ApiResponse)]
#[oai(bad_request_handler = "bad_request_handler")]
pub enum GenericResponse<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<ResponseObject<T>>),
    #[oai(status = 200)]
    Error(Json<ResponseObject<String>>),
}

impl<T: ParseFromJSON + ToJSON> GenericResponse<T> {
    pub fn ok(data: T) -> Self {
        GenericResponse::Ok(Json(ResponseObject {
            code: ResponseCode::Success as i32,
            msg: "Ok".to_string(),
            data: Some(data),
        }))
    }

    pub fn error(message: &str) -> Self {
        GenericResponse::Error(Json(ResponseObject::error(message)))
    }
}

fn bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(err: Error) -> GenericResponse<T> {
    if err.is::<ParseRequestPayloadError>() {
        GenericResponse::Ok(Json(ResponseObject {
            code: ResponseCode::InvalidRequest as i32,
            msg: err.to_string(),
            data: None,
        }))
    } else {
        GenericResponse::Ok(Json(ResponseObject {
            code: ResponseCode::Unknown as i32,
            msg: err.to_string(),
            data: None,
        }))
    }
}
