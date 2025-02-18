use abi::sea_orm::{self, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user-base")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: String,
    pub nikename: String,
    pub create_at: i64,
    pub update_at: i64,
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
