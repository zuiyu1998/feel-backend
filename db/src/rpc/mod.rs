mod service;

use std::sync::Arc;

use abi::{
    config::{Config, FromConfig, ServiceType},
    pb::types::db_service_server::DbServiceServer,
    tonic::{async_trait, transport::Server},
    tracing,
};

use utils::{
    helpers,
    synapse::health::{HealthServer, HealthService},
};

use crate::{database::DbRepo, Error, Result};

pub struct DbRpcService {
    db: Arc<DbRepo>,
}

impl DbRpcService {
    pub async fn start(config: &Config) -> Result<()> {
        // register service
        helpers::register_service(config, ServiceType::Db).await?;
        tracing::info!("<db> rpc service health check started");

        let health_service = HealthServer::new(HealthService {});
        tracing::info!("<db> rpc service register to service register center");

        let db_rpc = DbRpcService::from_config(config).await?;
        let service = DbServiceServer::new(db_rpc);
        tracing::info!(
            "<db> rpc service started at {}",
            config.rpc.db.rpc_server_url()
        );

        Server::builder()
            .add_service(health_service)
            .add_service(service)
            .serve(config.rpc.db.rpc_server_url().parse().unwrap())
            .await?;

        Ok(())
    }
}

#[async_trait]
impl FromConfig for DbRpcService {
    type Error = Error;

    async fn from_config(config: &Config) -> Result<Self> {
        let db = Arc::new(DbRepo::from_config(config).await?);

        Ok(DbRpcService { db })
    }
}
