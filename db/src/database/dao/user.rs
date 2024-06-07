use crate::{database::UserRepo, Result};
use sea_orm::DatabaseConnection;

use abi::{
    pb::types::{GetUserInfoParam, UserBase, UserLogin, UserRegister, UserUnregister, UserUpdate},
    tonic::async_trait,
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
    async fn register(&self, _register: UserRegister) -> Result<UserBase> {
        todo!()
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
