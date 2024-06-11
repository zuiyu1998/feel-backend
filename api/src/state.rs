use std::sync::Arc;

use abi::{
    config::{Config, FromConfig},
    pb::types::db_service_client::DbServiceClient,
    tonic::async_trait,
};
use utils::service_discovery::LbWithServiceDiscovery;

use crate::{helpers::JwtHelper, Error};

#[derive(Clone)]
pub struct AppState {
    pub db_rpc: DbServiceClient<LbWithServiceDiscovery>,
    pub jwt_helper: Arc<JwtHelper>,
}

#[async_trait]
impl FromConfig for AppState {
    type Error = Error;

    async fn from_config(config: &Config) -> Result<Self, Self::Error> {
        let db_rpc = utils::helpers::get_rpc_client(&config, &config.rpc.db.name).await?;

        let jwt_helper = Arc::new(JwtHelper::from_config(config).await?);

        Ok(AppState { db_rpc, jwt_helper })
    }
}
