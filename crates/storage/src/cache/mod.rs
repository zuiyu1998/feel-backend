pub mod redis_impl;

use crate::user::dto::*;
use abi::{tonic::async_trait, Result};

#[async_trait]
pub trait Cache: 'static + Send + Sync {
    //获取用户信息
    async fn get_user_base(&self, id: i64) -> Result<Option<User>>;

    //插入用户信息
    async fn insert_user_base(&self, id: i64, user: User) -> Result<()>;

    //获取用户授权
    async fn get_user_auth(
        &self,
        auth_type: UserAuthType,
        auth_name: &str,
    ) -> Result<Option<UserAuth>>;

    //插入用户授权
    async fn insert_user_auth(
        &self,
        auth_type: UserAuthType,
        auth_name: &str,
        user_auth: UserAuth,
    ) -> Result<()>;
}
