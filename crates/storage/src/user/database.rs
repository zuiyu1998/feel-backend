use std::sync::Arc;

use abi::{tonic::async_trait, Result};

use crate::{
    cache::Cache,
    user::{dto::*, UserRepo},
};

use super::UserDataBase;

pub struct UserDataBaseImpl {
    db: Arc<dyn UserRepo>,
    cache: Arc<dyn Cache>,
}

impl UserDataBaseImpl {
    pub fn new(db: Arc<dyn UserRepo>, cache: Arc<dyn Cache>) -> Self {
        UserDataBaseImpl { db, cache }
    }
}

#[async_trait]
impl UserDataBase for UserDataBaseImpl {
    ///注册
    ///todo 错误处理
    async fn register(&self, form: &RegisterUserForm) -> Result<User> {
        let user = self.db.register(form).await?;
        Ok(user)
    }

    //注销
    async fn unregister(&self, user_id: i64) -> Result<()> {
        self.db.unregister(user_id).await?;

        Ok(())
    }

    async fn get_user_base(&self, id: i64) -> Result<User> {
        if let Some(user) = self.cache.get_user_base(id).await? {
            return Ok(user);
        }

        let user = self.db.get_user_base(id).await?;

        self.cache.insert_user_base(id, user.clone()).await?;

        Ok(user)
    }

    async fn get_user_auth(&self, auth_type: UserAuthType, auth_name: &str) -> Result<UserAuth> {
        if let Some(auth) = self
            .cache
            .get_user_auth(auth_type.clone(), auth_name)
            .await?
        {
            return Ok(auth);
        }

        let auth = self.db.get_user_auth(auth_type.clone(), auth_name).await?;
        self.cache
            .insert_user_auth(auth_type, auth_name, auth.clone())
            .await?;

        Ok(auth)
    }
}
