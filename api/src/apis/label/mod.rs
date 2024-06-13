mod dto;

use abi::tonic::Request;
use poem::{error::Result, web::Data};
use poem_openapi::{payload::Json, OpenApi};
use validator::Validate;

use super::utils::*;
use crate::{state::AppState, Error};

use dto::*;

pub struct LabelApi;

#[OpenApi(prefix_path = "/label", tag = "super::ApiTags::LabelApi")]
impl LabelApi {
    #[oai(path = "/create_lable_meta", method = "post")]
    async fn create_lable_meta(
        &self,
        Data(state): Data<&AppState>,
        Json(req): Json<UserLabelMetaCreateRequest>,
    ) -> Result<GenericResponse<Empty>> {
        req.validate()
            .map_err(|e| Error::RequestError(e.to_string()))?;

        let mut db_rpc = state.db_rpc.clone();

        db_rpc
            .create_lable_meta(Request::new(req.into_inner()))
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?;

        Ok(GenericResponse::ok(Empty))
    }

    #[oai(path = "/create_user_lable", method = "post")]
    async fn create_user_lable(
        &self,
        Data(state): Data<&AppState>,
        Json(req): Json<UserLabelCreateRequest>,
    ) -> Result<GenericResponse<Empty>> {
        req.validate()
            .map_err(|e| Error::RequestError(e.to_string()))?;

        let mut db_rpc = state.db_rpc.clone();

        db_rpc
            .create_user_lable(Request::new(req.into_inner()))
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?;

        Ok(GenericResponse::ok(Empty))
    }
}
