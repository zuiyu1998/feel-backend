use abi::pb::types::AuthType;
use validator::{
    Validate, ValidateEmail, ValidateLength, ValidateUrl, ValidationError, ValidationErrors,
};

pub fn validate_nikename(target: &str) -> Result<(), ValidationErrors> {
    if !target.validate_length(Some(2), Some(16), None) {
        return Err(get_validation_errors(
            "nikename",
            "nikename length too min or too max",
        ));
    } else {
        return Ok(());
    }
}

pub fn validate_auth(auth_type: &str, auth_name: &str) -> Result<(), ValidationErrors> {
    if let Some(auth) = AuthType::from_str_name(auth_type) {
        match auth {
            AuthType::Email => {
                if !auth_name.validate_email() {
                    return Err(get_validation_errors(
                        "auth_name",
                        "auth_name is invaild email",
                    ));
                } else {
                    Ok(())
                }
            }
        }
    } else {
        return Err(get_validation_errors("auth_type", "auth_type is invaild"));
    }
}

pub fn validate_avatar(target: &str) -> Result<(), ValidationErrors> {
    if !ValidateUrl::validate_url(target) {
        return Err(get_validation_errors("avatar", "avatar is invaild"));
    } else {
        return Ok(());
    }
}

fn get_validation_errors(field: &'static str, code: &'static str) -> ValidationErrors {
    let mut errors = ValidationErrors::new();

    errors.add(&field, ValidationError::new(code));

    errors
}
