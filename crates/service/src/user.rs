use std::sync::Arc;

use abi::{
    protocol::pb::feel_sdk::{
        user_service_server::UserService, RegisterUserReq, RegisterUserResp, UnregisterUserReq,
        UnregisterUserResp, UserLoginReq, UserLoginResp,
    },
    tonic::{async_trait, Request, Response, Status},
    ErrorKind, Result,
};
use db::user::UserDataBase;
use tools::encryptor::sha2;

pub struct UserServiceImpl {
    database: Arc<dyn UserDataBase>,
}

impl UserServiceImpl {
    pub fn new(database: Arc<dyn UserDataBase>) -> UserServiceImpl {
        Self { database }
    }
}

impl UserServiceImpl {
    pub async fn register_user(&self, req: RegisterUserReq) -> Result<RegisterUserResp> {
        let form = req.into();

        let user = self.database.register(&form).await?;

        Ok(RegisterUserResp {
            id: user.id,
            uid: user.uid,
            nikename: user.nikename,
        })
    }

    pub async fn unregister_user(&self, _req: UnregisterUserReq) -> Result<UnregisterUserResp> {
        todo!()
    }

    pub async fn user_login(&self, req: UserLoginReq) -> Result<UserLoginResp> {
        let auth = self
            .database
            .get_user_auth(req.auth_type.into(), &req.auth_name)
            .await?;
        let auth_token = sha2(auth.salt.as_bytes(), &req.auth_token.as_bytes());

        if auth_token != auth.auth_token {
            return Err(ErrorKind::PasswordNotMatch.into());
        }

        let user = self.database.get_user_base(auth.user_id).await?;

        Ok(user.into_user_login_resp())
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn register_user(
        &self,
        request: Request<RegisterUserReq>,
    ) -> Result<Response<RegisterUserResp>, Status> {
        let req = request.into_inner();

        let resp = self.register_user(req).await?;

        Ok(Response::new(resp))
    }

    async fn unregister_user(
        &self,
        request: Request<UnregisterUserReq>,
    ) -> Result<Response<UnregisterUserResp>, Status> {
        let req = request.into_inner();

        let resp = self.unregister_user(req).await?;

        Ok(Response::new(resp))
    }
    async fn user_login(
        &self,
        request: Request<UserLoginReq>,
    ) -> Result<Response<UserLoginResp>, Status> {
        let req = request.into_inner();

        let resp = self.user_login(req).await?;

        Ok(Response::new(resp))
    }
}
