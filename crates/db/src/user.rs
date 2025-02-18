use abi::{tonic::async_trait, Result};

pub mod dto;

use dto::*;

///用户系统
#[async_trait]
pub trait UserRepo: 'static + Send + Sync {
    //创建用户
    async fn register(&self, form: &RegisterUserForm) -> Result<User>;

    //注销
    async fn unregister(&self, user_id: i64) -> Result<()>;

    //获取用户信息
    async fn get_user_base(&self, uid: &str) -> Result<User>;

    //获取用户授权
    async fn get_user_auth(&self, uid: &str, auth_type: UserAuthType) -> Result<UserAuth>;
}
