use abi::pb::types::{AuthType, UserRegister};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::{
    Validate, ValidateEmail, ValidateLength, ValidateUrl, ValidationError, ValidationErrors,
};

#[derive(Deserialize, Serialize, Object)]
pub struct UserRegisterRequest {
    pub nikename: String,
    pub avatar: String,
    pub uid: String,
    pub auth_type: String,
    pub auth_name: String,
    pub auth_code: String,
}

fn get_validation_errors(field: &'static str, code: &'static str) -> ValidationErrors {
    let mut errors = ValidationErrors::new();

    errors.add(&field, ValidationError::new(code));

    errors
}

impl Validate for UserRegisterRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if !self.nikename.validate_length(Some(2), Some(16), None) {
            return Err(get_validation_errors(
                "nikename",
                "nikename length too min or too max",
            ));
        }

        if !self.avatar.validate_url() {
            return Err(get_validation_errors("avatar", "avatar is invaild"));
        }

        if let Some(auth) = AuthType::from_str_name(&self.auth_type) {
            match auth {
                AuthType::Email => {
                    if !self.auth_name.validate_email() {
                        return Err(get_validation_errors(
                            "auth_name",
                            "auth_name is invaild email",
                        ));
                    }
                }
            }
        } else {
            return Err(get_validation_errors("auth_type", "auth_type is invaild"));
        }

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
