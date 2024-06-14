mod database;
mod error;

pub mod helpers;
pub mod rpc;

pub use error::*;

use abi::sea_orm::DatabaseBackend;

pub const DATABASE_BACKEND: DatabaseBackend = DatabaseBackend::Postgres;

use abi::config::Config;
use rpc::DbRpcService;

pub async fn start_server(config: &Config) -> Result<()> {
    DbRpcService::start(&config).await?;

    Ok(())
}
