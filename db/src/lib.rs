mod database;
mod error;
mod rpc;

use abi::config::Config;
pub use entity::sea_orm;
pub use error::*;

use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

pub async fn start_server() -> Result<()> {
    let config = Config::load()?;

    let connction = Database::connect(&config.db.postgres).await?;

    Migrator::up(&connction, None).await?;

    Ok(())
}
