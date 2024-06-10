use crate::sea_orm::DatabaseConnection;
use crate::{database::UserRepo, Result};

use abi::{
    pb::types::{GetUserInfoParam, UserBase, UserLogin, UserRegister, UserUnregister, UserUpdate},
    tonic::async_trait,
};
use entity::{
    sea_orm::{ActiveModelTrait, IntoActiveModel},
    user::{UserAuthActiveModel, UserBaseActiveModel},
};

#[derive(Debug)]
pub struct DaoUser {
    connection: DatabaseConnection,
}

impl DaoUser {
    pub fn new(connection: DatabaseConnection) -> Self {
        DaoUser { connection }
    }
}

#[async_trait]
impl UserRepo for DaoUser {
    async fn register(&self, register: UserRegister) -> Result<UserBase> {
        let user_model: UserBaseActiveModel = register.clone().into_active_model();

        let user_model = user_model.insert(&self.connection).await?;

        let auth_model: UserAuthActiveModel = register.into_active_model();

        let _auth_model = auth_model.insert(&self.connection).await?;

        Ok(UserBase::from(user_model))
    }
    async fn unregister(&self, _unregister: UserUnregister) -> Result<UserBase> {
        todo!()
    }
    async fn login(&self, _login: UserLogin) -> Result<UserBase> {
        todo!()
    }
    async fn get_user_info(&self, _param: GetUserInfoParam) -> Result<UserBase> {
        todo!()
    }
    async fn update(&self, _update: UserUpdate) -> Result<UserBase> {
        todo!()
    }
}
