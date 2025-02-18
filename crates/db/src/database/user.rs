use abi::{
    sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel, Set},
    tonic::async_trait,
    Result,
};
use entity::user::*;

use crate::user::{dto::*, UserRepo};

pub struct UserDataBase<C> {
    conn: C,
}

impl<C: ConnectionTrait> UserDataBase<C> {
    pub fn new(conn: C) -> UserDataBase<C> {
        UserDataBase { conn }
    }
}

#[async_trait]
impl<C: ConnectionTrait + Send + 'static> UserRepo for UserDataBase<C> {
    ///注册
    ///todo 错误处理
    async fn register(&self, form: &RegisterUserForm) -> Result<User> {
        let user_active: UserBaseActiveModel = form.clone().into_active_model();

        let user_model = user_active.insert(&self.conn).await?;

        let auth_active = form.get_user_auth_active_model(user_model.id);

        auth_active.insert(&self.conn).await?;

        Ok(User::new(user_model))
    }

    //注销
    async fn unregister(&self, user_id: i64) -> Result<()> {
        let mut user_active: UserBaseActiveModel = ActiveModelTrait::default();
        user_active.id = Set(user_id);
        user_active.is_delete = Set(true);

        user_active.update(&self.conn).await?;

        Ok(())
    }

    async fn get_user_base(&self, _uid: &str) -> Result<User> {
        todo!()
    }

    async fn get_user_auth(&self, _uid: &str, _auth_type: UserAuthType) -> Result<UserAuth> {
        todo!()
    }
}
