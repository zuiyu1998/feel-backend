mod dto;

use abi::tonic::Request;
use poem::{error::Result, web::Data};
use poem_openapi::{payload::Json, OpenApi};
use validator::Validate;

use super::utils::*;
use crate::{state::AppState, Error};

use dto::*;

pub struct UserApi;

#[OpenApi(prefix_path = "/user")]
impl UserApi {
    #[oai(path = "/register", method = "post")]
    async fn register(
        &self,
        Data(state): Data<&AppState>,
        Json(register): Json<UserRegisterRequest>,
    ) -> Result<GenericResponse<Empty>> {
        register
            .validate()
            .map_err(|e| Error::RequestError(e.to_string()))?;

        let mut db_rpc = state.db_rpc.clone();

        db_rpc
            .register(Request::new(register.into_inner()))
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?;

        Ok(GenericResponse::ok(Empty))
    }
}
