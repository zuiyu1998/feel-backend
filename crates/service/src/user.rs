use std::sync::Arc;

use abi::{async_trait::async_trait, user::*, Result};
use db::user::UserRepo;

use crate::encryptor::Encryptor;

pub struct UserRegisterReq {
    pub login_type: LoginType,
    pub auth_token: String,
    pub auth_name: String,
    pub nikename: String,
    pub uid: String,
}

impl UserRegisterReq {
    pub fn to_form(&self, encryptor: &Arc<dyn Encryptor>) -> UserRegisterForm {
        let auth_token = encryptor.encode(&self.auth_token.as_bytes());

        UserRegisterForm {
            login_type: self.login_type.clone(),
            auth_token,
            auth_name: self.auth_name.clone(),
            nikename: self.nikename.clone(),
            uid: self.uid.clone(),
            create_at: 0,
            update_at: 0,
        }
    }
}

#[async_trait]
pub trait UserService: 'static + Send + Sync {
    async fn register(&self, req: &UserRegisterReq) -> Result<()>;
}

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepo>,
    encryptor: Arc<dyn Encryptor>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn register(&self, req: &UserRegisterReq) -> Result<()> {
        let form = req.to_form(&self.encryptor);

        self.user_repo.register(&form).await?;

        Ok(())
    }
}
