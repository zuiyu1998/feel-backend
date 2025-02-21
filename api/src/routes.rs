use std::sync::Arc;

use abi::{log::LogConfig, sea_orm::Database, Result};
use axum::Router;
use db::{
    cache::redis_impl::{RedisCache, RedisConfig},
    user::{db::UserDb, user::UserDataBaseImpl, UserDataBase}, DbConfig,
};
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use service::user::UserServiceImpl;
use tools::jwt_helper::{JWTConfig, JwtHelper};

use crate::user;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiConfig {
    pub api: ApiServerConfig,
    pub log: LogConfig,
    pub db: DbConfig,
    pub jwt: JWTConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiServerConfig {
    host: String,
    port: u16,
}

impl ApiServerConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserServiceImpl>,
    pub jwt_helper: Arc<JwtHelper>,
}

impl AppState {
    pub fn new(database: Arc<dyn UserDataBase>, jwt_helper: JwtHelper) -> Self {
        AppState {
            user_service: Arc::new(UserServiceImpl::new(database)),
            jwt_helper: Arc::new(jwt_helper),
        }
    }

    pub async fn from_config(config: &ApiConfig) -> Result<Self> {
        let conn = Database::connect(&config.db.database_url).await?;
        let jwt_helper = JwtHelper::from_config(&config.jwt);

        let use_repo = Arc::new(UserDb::new(conn.clone()));
        let cache = Arc::new(RedisCache::from_config(&config.redis)?);

        let user_database = Arc::new(UserDataBaseImpl::new(use_repo, cache));

        Migrator::up(&conn, None).await?;

        Ok(AppState::new(user_database, jwt_helper))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().nest("/v1/user", user::routes())
}
