use std::sync::Arc;

use abi::{
    config::Config,
    sea_orm::{Database, DatabaseConnection},
    Result,
};
use axum::Router;
use migration::{Migrator, MigratorTrait};
use service::user::UserServiceImpl;

use crate::user;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserServiceImpl>,
}

impl AppState {
    pub fn new(conn: DatabaseConnection) -> Self {
        AppState {
            user_service: Arc::new(UserServiceImpl::new(&conn)),
        }
    }

    pub async fn from_config(config: &Config) -> Result<Self> {
        let conn = Database::connect(&config.db.database_url).await?;

        Migrator::up(&conn, None).await?;

        Ok(AppState::new(conn))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().nest("/v1/user", user::routes())
}
