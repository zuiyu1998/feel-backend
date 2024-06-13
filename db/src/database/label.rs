use crate::Result;
use abi::{
    pb::types::{
        UserLabelCreate, UserLabelCreateResponse, UserLabelMetaCreate, UserLabelMetaCreateResponse,
        UserLabelParams, UserLabelResponse,
    },
    tonic::async_trait,
};
use std::fmt::Debug;

#[async_trait]
pub trait LabelRepo: Sync + Send + Debug {
    async fn get_user_labels(&self, params: UserLabelParams) -> Result<UserLabelResponse>;
    async fn create_user_lable(&self, create: UserLabelCreate) -> Result<UserLabelCreateResponse>;
    async fn create_lable_meta(
        &self,
        create: UserLabelMetaCreate,
    ) -> Result<UserLabelMetaCreateResponse>;
}
