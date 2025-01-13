use abi::{async_trait::async_trait, sea_orm::DatabaseConnection, user::*, Result};

use crate::user::UserRepo;

pub struct UserDb {
    conn: DatabaseConnection,
}

#[async_trait]
impl UserRepo for UserDb {
    //注册
    async fn register(&self, form: &UserRegisterForm) -> Result<()> {
        todo!()
    }

    //注销
    async fn unregister(&self, form: &UserUnregisterForm) -> Result<()> {
        todo!()
    }

    ///登录
    async fn login(&self, form: &UserLoginForm) -> Result<()> {
        todo!()
    }
}
