use abi::{
    sea_orm::{self, entity::prelude::*, IntoActiveModel, Set},
    user::UserAuthRegisterForm,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user-auth")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i64,
    #[sea_orm(primary_key)]
    pub login_type: String,
    pub auth_token: String,
    pub auth_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UserAuthRegisterForm {
    fn into_active_model(self) -> ActiveModel {
        let mut model: ActiveModel = <ActiveModel as ActiveModelTrait>::default();

        model.user_id = Set(self.user_id);
        model.login_type = Set(self.login_type.as_str().to_string());
        model.auth_token = Set(self.auth_token);
        model.auth_name = Set(self.auth_name);

        model
    }
}
