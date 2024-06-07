use crate::service_discovery::LbWithServiceDiscovery;

use abi::pb::types::db_service_client::DbServiceClient;

pub trait ClientFactory {
    // 获取对应的grpc客户端
    fn new_client(channel: LbWithServiceDiscovery) -> Self;
}

impl ClientFactory for DbServiceClient<LbWithServiceDiscovery> {
    fn new_client(channel: LbWithServiceDiscovery) -> Self {
        Self::new(channel)
    }
}
