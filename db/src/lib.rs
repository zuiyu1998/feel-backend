mod database;
mod error;

pub mod helpers;
pub mod rpc;

use abi::config::Config;
pub use entity::sea_orm;
pub use error::*;
use rpc::DbRpcService;

pub async fn start_server(config: &Config) -> Result<()> {
    DbRpcService::start(&config).await?;

    Ok(())
}
