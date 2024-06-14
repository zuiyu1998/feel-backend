use serde::{Deserialize, Serialize};

use abi::{
    chrono::NaiveDateTime,
    pb::types::UserLabelMetaCreate,
    sea_orm::{self, entity::prelude::*, IntoActiveModel, Set},
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "label_meta")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub effct: i64,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UserLabelMetaCreate {
    fn into_active_model(self) -> ActiveModel {
        let mut model: ActiveModel = Default::default();

        model.name = Set(self.name);
        model.description = Set(self.description);
        model.effct = Set(self.effct);

        model
    }
}
