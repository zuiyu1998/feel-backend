use crate::Result;
use abi::{
    pb::types::{GetUserInfoParam, UserBase, UserLogin, UserRegister, UserUnregister, UserUpdate},
    tonic::async_trait,
};
use std::fmt::Debug;

#[async_trait]
pub trait UserRepo: Sync + Send + Debug {
    async fn register(&self, register: UserRegister) -> Result<UserBase>;
    async fn unregister(&self, unregister: UserUnregister) -> Result<UserBase>;
    async fn login(&self, login: UserLogin) -> Result<UserBase>;
    async fn get_user_info(&self, param: GetUserInfoParam) -> Result<UserBase>;
    async fn update(&self, update: UserUpdate) -> Result<UserBase>;
}