mod dto;

use abi::{
    pb::types::{GetUserInfoParams, UserUnregister},
    tonic::Request,
};
use poem::{error::Result, web::Data};
use poem_openapi::{payload::Json, OpenApi};
use validator::Validate;

use super::utils::*;
use crate::{state::AppState, Error};

use dto::*;

pub struct UserApi;

#[OpenApi(prefix_path = "/user", tag = "super::ApiTags::UserApi")]
impl UserApi {
    #[oai(path = "/update_avatar", method = "post")]
    async fn update_avatar(
        &self,
        Data(state): Data<&AppState>,
        Json(req): Json<UserUpdateAvatarRequest>,
    ) -> Result<GenericResponse<Empty>> {
        req.validate()
            .map_err(|e| Error::RequestError(e.to_string()))?;

        let mut db_rpc = state.db_rpc.clone();

        db_rpc
            .update(Request::new(req.into_inner()))
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?;

        Ok(GenericResponse::ok(Empty))
    }

    #[oai(path = "/update_nikename", method = "post")]
    async fn update_nikename(
        &self,
        Data(state): Data<&AppState>,
        Json(req): Json<UserUpdateNikenameRequest>,
    ) -> Result<GenericResponse<Empty>> {
        req.validate()
            .map_err(|e| Error::RequestError(e.to_string()))?;

        let mut db_rpc = state.db_rpc.clone();

        db_rpc
            .update(Request::new(req.into_inner()))
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?;

        Ok(GenericResponse::ok(Empty))
    }

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

    #[oai(path = "/login", method = "post")]
    async fn login(
        &self,
        Data(state): Data<&AppState>,
        Json(login): Json<UserLoginRequest>,
    ) -> Result<GenericResponse<String>> {
        login
            .validate()
            .map_err(|e| Error::RequestError(e.to_string()))?;

        let mut db_rpc = state.db_rpc.clone();

        let user_base = db_rpc
            .login(Request::new(login.into_inner()))
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?
            .into_inner();

        let token = state.jwt_helper.encode(&user_base.id.to_string())?;

        Ok(GenericResponse::ok(token))
    }

    #[oai(path = "/get_user_info", method = "post")]
    async fn get_user_info(
        &self,
        Data(state): Data<&AppState>,
        Authorization(user_id): Authorization,
    ) -> Result<GenericResponse<UserBaseResponse>> {
        let mut db_rpc = state.db_rpc.clone();

        let user_base = db_rpc
            .get_user_info(Request::new(GetUserInfoParams { id: user_id }))
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?
            .into_inner();

        Ok(GenericResponse::ok(UserBaseResponse::from_user_base(
            user_base,
        )))
    }

    #[oai(path = "/unregister", method = "post")]
    async fn unregister(
        &self,
        Data(state): Data<&AppState>,
        Authorization(user_id): Authorization,
    ) -> Result<GenericResponse<UserBaseResponse>> {
        let mut db_rpc = state.db_rpc.clone();

        let user_base = db_rpc
            .unregister(Request::new(UserUnregister { id: user_id }))
            .await
            .map_err(|e| Error::InternalServer(e.to_string()))?
            .into_inner();

        Ok(GenericResponse::ok(UserBaseResponse::from_user_base(
            user_base,
        )))
    }
}
