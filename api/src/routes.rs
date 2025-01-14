use std::sync::Arc;

use abi::{config::Config, Result};
use axum::Router;
use db::Db;
use service::{
    encryptor::{sha2_impl::Sha2Encryptor, Encryptor},
    user::{UserService, UserServiceImpl},
};

use crate::user;

#[derive(Clone)]
pub struct AppState {
   pub user_service: Arc<dyn UserService>,
}

impl AppState {
    pub fn new(db: Db, encryptor: Arc<dyn Encryptor>) -> Self {
        AppState {
            user_service: Arc::new(UserServiceImpl::new(db.user.clone(), encryptor)),
        }
    }

    pub async fn from_config(config: &Config) -> Result<Self> {
        let db: Db = Db::from_config(&config.db).await?;
        let encryptor = Sha2Encryptor::from_config(&config.sha);

        Ok(AppState::new(db, Arc::new(encryptor)))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().nest("/v1/user", user::routes())
}
