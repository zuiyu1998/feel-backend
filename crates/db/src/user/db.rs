use abi::{
    sea_orm::{
        ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, QueryFilter,
        Set,
    },
    tonic::async_trait,
    ErrorKind, Result,
};
use entity::user::*;

use crate::user::{dto::*, UserRepo};

pub struct UserDb<C> {
    conn: C,
}

impl<C: ConnectionTrait> UserDb<C> {
    pub fn new(conn: C) -> UserDb<C> {
        UserDb { conn }
    }
}

#[async_trait]
impl<C: ConnectionTrait + Send + 'static> UserRepo for UserDb<C> {
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

    async fn get_user_base(&self, id: i64) -> Result<User> {
        let sql = UserBaseEntity::find_by_id(id);

        let model = sql.one(&self.conn).await?.ok_or(ErrorKind::UserNotFound)?;
        Ok(User::from_user_model(model))
    }

    async fn get_user_auth(&self, auth_type: UserAuthType, auth_name: &str) -> Result<UserAuth> {
        let sql = UserAuthEntity::find()
            .filter(UserAuthColumn::AuthType.eq(auth_type.as_str()))
            .filter(UserAuthColumn::AuthName.eq(auth_name));

        let model = sql.one(&self.conn).await?.ok_or(ErrorKind::AuthNotFound)?;

        Ok(UserAuth::from_user_auth_model(model)?)
    }
}
