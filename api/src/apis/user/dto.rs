use abi::pb::types::{AuthType, UserBase, UserLogin, UserRegister, UserUpdate};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::apis::utils::{validate_auth, validate_avatar, validate_nikename};

#[derive(Deserialize, Serialize, Object)]
pub struct UserUpdateAvatarRequest {
    pub id: i32,
    pub avatar: String,
}

impl Validate for UserUpdateAvatarRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_avatar(&self.avatar)?;

        Ok(())
    }
}

impl UserUpdateAvatarRequest {
    pub fn into_inner(self) -> UserUpdate {
        UserUpdate {
            id: self.id,
            nikename: None,
            avatar: Some(self.avatar),
        }
    }
}

#[derive(Deserialize, Serialize, Object)]
pub struct UserUpdateNikenameRequest {
    pub id: i32,
    pub nikename: String,
}

impl Validate for UserUpdateNikenameRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_nikename(&self.nikename)?;

        Ok(())
    }
}

impl UserUpdateNikenameRequest {
    pub fn into_inner(self) -> UserUpdate {
        UserUpdate {
            id: self.id,
            nikename: Some(self.nikename),
            avatar: None,
        }
    }
}

#[derive(Deserialize, Serialize, Object)]
pub struct UserBaseResponse {
    pub id: i32,
    pub nikename: String,
    pub avatar: String,
    pub uid: String,
    pub create_at: i64,
    pub update_at: i64,
}

impl UserBaseResponse {
    pub fn from_user_base(user: UserBase) -> Self {
        let UserBase {
            id,
            nikename,
            avatar,
            uid,
            create_at,
            update_at,
        } = user;

        UserBaseResponse {
            id,
            nikename,
            avatar,
            uid,
            create_at,
            update_at,
        }
    }
}

#[derive(Deserialize, Serialize, Object)]
pub struct UserLoginRequest {
    /// 授权方式，目前仅实现了Email
    pub auth_type: String,
    pub auth_name: String,
    pub auth_code: String,
}

impl Validate for UserLoginRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_auth(&self.auth_type, &self.auth_name)?;

        Ok(())
    }
}

impl UserLoginRequest {
    pub fn into_inner(self) -> UserLogin {
        let UserLoginRequest {
            auth_type,
            auth_name,
            auth_code,
        } = self;

        UserLogin {
            auth_type: AuthType::from_str_name(&auth_type).unwrap() as i32,
            auth_name,
            auth_code,
        }
    }
}

#[derive(Deserialize, Serialize, Object)]
pub struct UserRegisterRequest {
    pub nikename: String,
    pub avatar: String,
    pub uid: String,
    /// 授权方式，目前仅实现了Email
    pub auth_type: String,
    pub auth_name: String,
    pub auth_code: String,
}

impl Validate for UserRegisterRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_nikename(&self.nikename)?;
        validate_avatar(&self.avatar)?;
        validate_auth(&self.auth_type, &self.auth_name)?;

        Ok(())
    }
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
