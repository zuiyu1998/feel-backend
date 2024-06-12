use crate::{helpers::ShaHelper, Result};
use abi::{
    pb::types::{GetUserInfoParams, UserBase, UserLogin, UserRegister, UserUnregister, UserUpdate},
    tonic::async_trait,
};
use std::fmt::Debug;

#[async_trait]
pub trait UserRepo: Sync + Send + Debug {
    async fn register(&self, register: UserRegister, sha_helper: &ShaHelper) -> Result<UserBase>;
    async fn unregister(&self, unregister: UserUnregister) -> Result<UserBase>;
    async fn login(&self, login: UserLogin, sha_helper: &ShaHelper) -> Result<UserBase>;
    async fn get_user_info(&self, param: GetUserInfoParams) -> Result<UserBase>;
    async fn update(&self, update: UserUpdate) -> Result<UserBase>;
}
