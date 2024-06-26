use abi::pb::types::UserRegister;
use abi::sea_orm::{self, entity::prelude::*, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_auth")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub auth_type: i32,
    pub auth_name: String,
    pub auth_code: Vec<u8>,
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UserRegister {
    fn into_active_model(self) -> ActiveModel {
        let mut model: ActiveModel = Default::default();

        model.auth_type = Set(self.auth_type);
        model.auth_name = Set(self.auth_name);

        model
    }
}
