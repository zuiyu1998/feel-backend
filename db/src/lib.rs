mod database;
mod error;
mod rpc;

use abi::config::{Config, FromConfig};
pub use entity::sea_orm;
pub use error::*;

pub async fn start_server() -> Result<()> {
    let config = Config::load()?;

    database::DbRepo::from_config(&config).await?;

    Ok(())
}
