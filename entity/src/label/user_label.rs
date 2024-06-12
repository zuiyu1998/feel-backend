use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use abi::chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub label_id: i32,
    pub user_id: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
