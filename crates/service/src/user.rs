use abi::{
    protocol::pb::feel_sdk::{
        user_service_server::UserService, RegisterUserReq, RegisterUserResp, UnregisterUserReq,
        UnregisterUserResp, UserLoginReq, UserLoginResp,
    },
    sea_orm::DatabaseConnection,
    tonic::{async_trait, Request, Response, Status},
    Result,
};
use db::{database::user::UserDataBase, user::UserRepo};

pub struct UserServiceImpl {
    database: UserDataBase<DatabaseConnection>,
}

impl UserServiceImpl {
    pub fn new(conn: &DatabaseConnection) -> UserServiceImpl {
        UserServiceImpl {
            database: UserDataBase::new(conn.clone()),
        }
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

    pub async fn user_login(&self, _req: UserLoginReq) -> Result<UserLoginResp> {
        todo!()
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
