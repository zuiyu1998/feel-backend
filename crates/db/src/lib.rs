use std::sync::Arc;

use abi::{config::DbConfig, sea_orm::Database, Result};
use database::UserDb;
use migration::{Migrator, MigratorTrait};
use user::UserRepo;

pub mod user;

pub mod database;

#[derive(Clone)]
pub struct Db {
   pub user: Arc<dyn UserRepo>,
}

impl Db {
    pub async fn from_config(config: &DbConfig) -> Result<Self> {
        let conn = Database::connect(&config.database_url).await?;

        Migrator::up(&conn, None).await?;

        Ok(Db {
            user: Arc::new(UserDb::new(conn)),
        })
    }
}
