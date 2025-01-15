use std::sync::Arc;

use crate::{encryptor::Encryptor, jwt_helper::JwtHelper};
use abi::{async_trait::async_trait, user::*, ErrorKind, Result};
use db::user::UserRepo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginReq {
    pub login_type: LoginType,
    pub auth_token: String,
    pub auth_name: String,
}

impl UserLoginReq {
    pub fn to_form(&self, encryptor: &Arc<dyn Encryptor>) -> UserLoginForm {
        let auth_token = encryptor.encode(&self.auth_token.as_bytes());

        UserLoginForm {
            login_type: self.login_type.clone(),
            auth_token: auth_token,
            auth_name: self.auth_name.clone(),
        }
    }
}

#[async_trait]
pub trait UserService: 'static + Send + Sync {
    async fn register(&self, req: &UserRegisterReq) -> Result<()>;
    async fn unregister(&self, token: &str) -> Result<()>;
    async fn login(&self, req: UserLoginReq) -> Result<String>;
}

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepo>,
    encryptor: Arc<dyn Encryptor>,
    jwt: Arc<JwtHelper>,
}

impl UserServiceImpl {
    pub fn new(
        user_repo: Arc<dyn UserRepo>,
        encryptor: Arc<dyn Encryptor>,
        jwt: Arc<JwtHelper>,
    ) -> Self {
        Self {
            user_repo,
            encryptor,
            jwt,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn register(&self, req: &UserRegisterReq) -> Result<()> {
        let form = req.to_form(&self.encryptor);

        self.user_repo.register(&form).await?;

        Ok(())
    }

    async fn unregister(&self, token: &str) -> Result<()> {
        if let Some(user_id) = self.jwt.decode(token) {
            self.user_repo.unregister(user_id).await?;
            Ok(())
        } else {
            Err(ErrorKind::TokenInvaild.into())
        }
    }

    async fn login(&self, req: UserLoginReq) -> Result<String> {
        let form = req.to_form(&self.encryptor);

        let user = self.user_repo.login(&form).await?;

        let token = self.jwt.encode(user.id);

        Ok(token)
    }
}
