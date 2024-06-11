use abi::{
    pb::types::{
        db_service_server::DbService, GetUserInfoParam, UserBase, UserLogin, UserRegister,
        UserUnregister, UserUpdate,
    },
    tonic::{async_trait, Request, Response, Status},
};

use super::DbRpcService;

#[async_trait]
impl DbService for DbRpcService {
    async fn register(&self, request: Request<UserRegister>) -> Result<Response<UserBase>, Status> {
        let req = request.into_inner();

        let user = self.db.user.register(req, &self.sha_helper).await?;

        Ok(Response::new(user))
    }

    async fn unregister(
        &self,
        request: Request<UserUnregister>,
    ) -> Result<Response<UserBase>, Status> {
        let req = request.into_inner();
        let user = self.db.user.unregister(req).await?;

        Ok(Response::new(user))
    }

    async fn login(&self, request: Request<UserLogin>) -> Result<Response<UserBase>, Status> {
        let req = request.into_inner();
        let user = self.db.user.login(req, &self.sha_helper).await?;

        Ok(Response::new(user))
    }

    async fn get_user_info(
        &self,
        request: Request<GetUserInfoParam>,
    ) -> Result<Response<UserBase>, Status> {
        let req = request.into_inner();
        let user = self.db.user.get_user_info(req).await?;

        Ok(Response::new(user))
    }

    async fn update(&self, request: Request<UserUpdate>) -> Result<Response<UserBase>, Status> {
        let req = request.into_inner();
        let user = self.db.user.update(req).await?;

        Ok(Response::new(user))
    }
}
