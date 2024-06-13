use crate::{database::LabelRepo, Result};

use abi::pb::types::{
    UserLabelCreate, UserLabelCreateResponse, UserLabelMetaCreate, UserLabelMetaCreateResponse,
    UserLabelParams, UserLabelResponse,
};
use abi::tonic::async_trait;
use entity::{
    label::{LabelMetaActiveModel, UserLabelActiveModel},
    sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel},
};

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
    async fn get_user_labels(&self, _params: UserLabelParams) -> Result<UserLabelResponse> {
        todo!()
    }

    async fn create_user_lable(&self, create: UserLabelCreate) -> Result<UserLabelCreateResponse> {
        let model: UserLabelActiveModel = create.into_active_model();

        let model = model.insert(&self.connection).await?;

        Ok(UserLabelCreateResponse { id: model.id })
    }
    async fn create_lable_meta(
        &self,
        create: UserLabelMetaCreate,
    ) -> Result<UserLabelMetaCreateResponse> {
        let model: LabelMetaActiveModel = create.into_active_model();

        let model = model.insert(&self.connection).await?;

        Ok(UserLabelMetaCreateResponse { id: model.id })
    }
}
