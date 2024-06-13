use crate::{database::LabelRepo, sea_orm::DatabaseConnection, Result};

use abi::pb::types::{
    UserLabel, UserLabelCreate, UserLabelMeta, UserLabelMetaCreate, UserLabelParams,
    UserLabelResponse,
};
use abi::tonic::async_trait;

#[derive(Debug)]
pub struct DaoLabel {
    connection: DatabaseConnection,
}

impl DaoLabel {
    pub fn new(connection: DatabaseConnection) -> Self {
        DaoLabel { connection }
    }
}

#[async_trait]
impl LabelRepo for DaoLabel {
    async fn get_user_labels(&self, params: UserLabelParams) -> Result<UserLabelResponse> {
        todo!()
    }

    async fn create_user_lable(&self, create: UserLabelCreate) -> Result<UserLabel> {
        todo!()
    }
    async fn create_lable_meta(&self, create: UserLabelMetaCreate) -> Result<UserLabelMeta> {
        todo!()
    }
}
