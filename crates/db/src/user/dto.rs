use abi::{
    protocol::pb::feel_sdk::{AuthType, RegisterUserReq},
    sea_orm::{ActiveModelTrait, IntoActiveModel, Set},
    tools::{
        encryptor::{self, get_rand_salt},
        time_util,
    },
};
use entity::user::*;

#[derive(Debug, Clone)]
pub struct RegisterUserForm {
    pub uid: String,
    pub nikename: String,
    pub auth_type: String,
    pub auth_token: String,
    pub auth_name: String,
}

impl From<RegisterUserReq> for RegisterUserForm {
    fn from(value: RegisterUserReq) -> Self {
        let auth_type = AuthType::try_from(value.auth_type).unwrap().as_str_name();

        Self {
            uid: value.uid,
            nikename: value.nikename,
            auth_type: auth_type.to_string(),
            auth_token: value.auth_token,
            auth_name: value.auth_name,
        }
    }
}

impl RegisterUserForm {
    pub fn get_user_auth_active_model(&self, user_id: i64) -> UserAuthActiveModel {
        let mut active: UserAuthActiveModel = <UserAuthActiveModel as ActiveModelTrait>::default();

        active.user_id = Set(user_id);
        active.auth_type = Set(self.auth_type.clone());
        active.auth_name = Set(self.auth_name.clone());

        let salt = get_rand_salt(20);
        let auth_token = encryptor::sha2(salt.as_bytes(), self.auth_token.as_bytes());

        active.salt = Set(salt);
        active.auth_token = Set(auth_token);

        active
    }
}

impl IntoActiveModel<UserBaseActiveModel> for RegisterUserForm {
    fn into_active_model(self) -> UserBaseActiveModel {
        let mut active: UserBaseActiveModel = <UserBaseActiveModel as ActiveModelTrait>::default();
        let now = time_util::now_timestamp();

        active.nikename = Set(self.nikename.clone());
        active.uid = Set(self.uid.clone());

        active.create_at = Set(now);
        active.update_at = Set(now);

        active.is_delete = Set(false);
        active.is_enable = Set(true);

        active
    }
}

pub struct User {
    pub id: i64,
    pub uid: String,
    pub nikename: String,
}

impl User {
    pub fn new(user_model: UserBaseModel) -> Self {
        User {
            id: user_model.id,
            uid: user_model.uid,
            nikename: user_model.nikename,
        }
    }
}

pub enum UserAuthType {}

pub struct UserAuth {}
