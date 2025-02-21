use std::sync::Arc;

use abi::{
    config::Config,
    sea_orm::{Database, DatabaseConnection},
    Result,
};
use axum::Router;
use migration::{Migrator, MigratorTrait};
use service::user::UserServiceImpl;
use tools::jwt_helper::JwtHelper;

use crate::user;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserServiceImpl>,
    pub jwt_helper: Arc<JwtHelper>,
}

impl AppState {
    pub fn new(conn: DatabaseConnection, jwt_helper: JwtHelper) -> Self {
        AppState {
            user_service: Arc::new(UserServiceImpl::new(&conn)),
            jwt_helper: Arc::new(jwt_helper),
        }
    }

    pub async fn from_config(config: &Config) -> Result<Self> {
        let conn = Database::connect(&config.db.database_url).await?;
        let jwt_helper = JwtHelper::from_config(&config.jwt);

        Migrator::up(&conn, None).await?;

        Ok(AppState::new(conn, jwt_helper))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().nest("/v1/user", user::routes())
}
