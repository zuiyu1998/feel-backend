mod database;
mod error;

pub mod helpers;
pub mod rpc;

pub use error::*;

use abi::config::Config;
use rpc::DbRpcService;

pub async fn start_server(config: &Config) -> Result<()> {
    DbRpcService::start(&config).await?;

    Ok(())
}
