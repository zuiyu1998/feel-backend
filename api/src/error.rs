use abi::Error;
use axum::{http::StatusCode, response::IntoResponse};

pub struct ApiError(Error);

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        ApiError(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::SERVICE_UNAVAILABLE, self.0.to_string()).into_response()
    }
}

pub type Result<T, E = ApiError> = std::result::Result<T, E>;
