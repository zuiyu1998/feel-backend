use abi::pb::types::{AuthType, UserRegister};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Object)]
pub struct UserRegisterRequest {
    pub nikename: String,
    pub avatar: String,
    pub uid: String,
    pub auth_type: String,
    pub auth_name: String,
    pub auth_code: String,
}

impl UserRegisterRequest {
    pub fn into_inner(self) -> UserRegister {
        let UserRegisterRequest {
            nikename,
            avatar,
            uid,
            auth_type,
            auth_name,
            auth_code,
        } = self;

        UserRegister {
            nikename,
            avatar,
            uid,
            auth_type: AuthType::from_str_name(&auth_type).unwrap() as i32,
            auth_name,
            auth_code,
        }
    }
}
