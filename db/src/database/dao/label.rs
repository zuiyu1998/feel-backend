use crate::{database::LabelRepo, Result, DATABASE_BACKEND};

use abi::pb::types::{
    UserLabel, UserLabelCreate, UserLabelCreateResponse, UserLabelMetaCreate,
    UserLabelMetaCreateResponse, UserLabelParams, UserLabelResponse,
};
use abi::sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QuerySelect, Statement,
};
use abi::tonic::async_trait;
use entity::label::{LabelMetaActiveModel, UserLabelActiveModel, UserLabelEntity};

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
        todo!();
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
