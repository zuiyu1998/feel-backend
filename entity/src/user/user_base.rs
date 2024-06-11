use abi::pb::types::{UserBase, UserRegister, UserUnregister, UserUpdate};
use sea_orm::{entity::prelude::*, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

use crate::utils::get_now;

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
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UserRegister {
    fn into_active_model(self) -> ActiveModel {
        let mut model: ActiveModel = Default::default();

        let now = get_now();

        model.nikename = Set(self.nikename);
        model.avatar = Set(self.avatar);
        model.uid = Set(self.uid);

        model.create_at = Set(now.clone());
        model.update_at = Set(now);

        model
    }
}

impl IntoActiveModel<ActiveModel> for UserUnregister {
    fn into_active_model(self) -> ActiveModel {
        let mut model: ActiveModel = Default::default();

        let now = get_now();

        model.is_delete = Set(true);
        model.is_enable = Set(false);
        model.id = Set(self.id);

        model.update_at = Set(now);

        model
    }
}

impl IntoActiveModel<ActiveModel> for UserUpdate {
    fn into_active_model(self) -> ActiveModel {
        let mut model: ActiveModel = Default::default();

        let now = get_now();

        if let Some(avatar) = self.avatar {
            model.avatar = Set(avatar);
        }

        if let Some(nikename) = self.nikename {
            model.nikename = Set(nikename);
        }

        model.id = Set(self.id);

        model.update_at = Set(now);

        model
    }
}

impl From<Model> for UserBase {
    fn from(value: Model) -> Self {
        UserBase {
            id: value.id,
            nikename: value.nikename,
            avatar: value.avatar,
            uid: value.uid,
            create_at: value.create_at.and_utc().timestamp(),
            update_at: value.update_at.and_utc().timestamp(),
        }
    }
}
