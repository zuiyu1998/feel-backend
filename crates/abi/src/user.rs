use async_trait::async_trait;

use crate::Result;

pub struct User {
    id: i64,
    uid: String,
    nikename: String,
    create_at: i64,
    update_at: i64,
}

///登录类型
pub enum LoginType {
    ///手机
    Phone,
}

pub struct UserLoginForm {
    login_type: LoginType,
    auth_token: String,
    auth_name: String,
}

pub struct UserRegisterForm {
    login_type: LoginType,
    auth_token: String,
    auth_name: String,
    nikename: String,
}

pub struct UserUnregisterForm {
    uid: String,
    token: String,
}

///用户系统
#[async_trait]
pub trait UserRepo {
    //注册
    async fn register(&self, form: &UserRegisterForm) -> Result<()>;

    //注销
    async fn unregister(&self, form: &UserUnregisterForm) -> Result<()>;

    ///登录
    async fn login(&self, form: &UserLoginForm) -> Result<()>;
}
