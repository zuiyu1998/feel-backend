use serde::{Deserialize, Serialize};

use abi::{
    chrono::NaiveDateTime,
    pb::types::UserLabelCreate,
    sea_orm::{self, entity::prelude::*, IntoActiveModel, Set},
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub label_meta_id: i32,
    pub user_id: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UserLabelCreate {
    fn into_active_model(self) -> ActiveModel {
        let mut model: ActiveModel = Default::default();

        model.user_id = Set(self.user_id);
        model.label_meta_id = Set(self.label_meta_id);

        model
    }
}
