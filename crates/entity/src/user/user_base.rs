use abi::{
    sea_orm::{self, entity::prelude::*, IntoActiveModel, Set},
    user::{User, UserRegisterForm},
};

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

impl IntoActiveModel<ActiveModel> for UserRegisterForm {
    fn into_active_model(self) -> ActiveModel {
        let mut model: ActiveModel = <ActiveModel as ActiveModelTrait>::default();

        model.nikename = Set(self.nikename);
        model.uid = Set(self.uid);
        model.create_at = Set(self.create_at);
        model.update_at = Set(self.update_at);

        model
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        User {
            id: value.id,
            nikename: value.nikename,
            uid: value.uid,
            create_at: value.create_at,
            update_at: value.update_at,
        }
    }
}
