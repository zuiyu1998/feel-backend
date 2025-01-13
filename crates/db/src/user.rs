use abi::{async_trait::async_trait, user::*, Result};

///用户系统
#[async_trait]
pub trait UserRepo: 'static + Send + Sync {
    //注册
    async fn register(&self, form: &UserRegisterForm) -> Result<()>;

    //注销
    async fn unregister(&self, form: &UserUnregisterForm) -> Result<()>;

    ///登录
    async fn login(&self, form: &UserLoginForm) -> Result<()>;

}
