use abi::{
    async_trait::async_trait,
    sea_orm::{
        ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
        QueryFilter, Set,
    },
    user::*,
    Result,
};
use entity::user::*;

use crate::user::UserRepo;

pub struct UserDb {
    conn: DatabaseConnection,
}

#[async_trait]
impl UserRepo for UserDb {
    ///注册
    ///todo 错误处理
    async fn register(&self, form: &UserRegisterForm) -> Result<()> {
        let user_active: UserBaseActiveModel = form.clone().into_active_model();

        let user_model = user_active.insert(&self.conn).await?;

        let auth_active = form.get_auth(user_model.id).into_active_model();

        auth_active.insert(&self.conn).await?;

        Ok(())
    }

    //注销
    async fn unregister(&self, user_id: i64) -> Result<()> {
        let mut user_active: UserBaseActiveModel = ActiveModelTrait::default();
        user_active.id = Set(user_id);
        user_active.is_delete = Set(true);

        user_active.update(&self.conn).await?;

        Ok(())
    }

    ///登录
    ///todo 错误处理
    async fn login(&self, form: &UserLoginForm) -> Result<User> {
        let auth_sql = UserAuthEntity::find()
            .filter(UserAuthColumn::LoginType.eq(form.login_type.as_str()))
            .filter(UserAuthColumn::AuthToken.eq(&form.auth_token))
            .filter(UserAuthColumn::AuthName.eq(&form.auth_name));

        let auth_model = auth_sql.one(&self.conn).await?.unwrap();

        let user_model = UserBaseEntity::find_by_id(auth_model.user_id)
            .one(&self.conn)
            .await?
            .unwrap();

        Ok(user_model.into())
    }
}
