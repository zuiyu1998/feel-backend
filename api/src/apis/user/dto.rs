use abi::pb::types::{AuthType, UserRegister};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserRegisterRequest {
    pub nikename: String,
    pub avatar: String,
    pub uid: String,
    pub auth_type: AuthType,
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
            auth_type: auth_type.into(),
            auth_name,
            auth_code,
        }
    }
}
