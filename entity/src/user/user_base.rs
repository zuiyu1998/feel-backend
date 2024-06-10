use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_base")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nikename: String,
    pub avatar: String,
    pub uid: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
