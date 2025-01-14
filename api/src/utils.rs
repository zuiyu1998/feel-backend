use std::error::Error;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AppCode {
    Ok = 200,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppJson<T = ()> {
    data: T,
    code: AppCode,
    message: String,
}

impl<T: Serialize> IntoResponse for AppJson<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub fn app_error<E: Error>(error: E) -> AppJson<()> {
    AppJson {
        data: (),
        code: AppCode::Ok,
        message: error.to_string(),
    }
}

impl<T: Serialize> AppJson<T> {
    pub fn ok(data: T) -> Self {
        AppJson {
            data,
            code: AppCode::Ok,
            message: "ok".to_string(),
        }
    }
}
