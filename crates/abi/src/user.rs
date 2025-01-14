pub struct User {
    pub id: i64,
    pub uid: String,
    pub nikename: String,
    pub create_at: i64,
    pub update_at: i64,
}

///登录类型
#[derive(Debug, Clone)]
pub enum LoginType {
    ///手机
    Phone,
}

impl LoginType {
    pub fn as_str(&self) -> &str {
        match self {
            LoginType::Phone => "Phone",
        }
    }
}

pub struct UserLoginForm {
    pub login_type: LoginType,
    pub auth_token: String,
    pub auth_name: String,
}

#[derive(Debug, Clone)]
pub struct UserRegisterForm {
    pub login_type: LoginType,
    pub auth_token: String,
    pub auth_name: String,
    pub nikename: String,
    pub uid: String,
    pub create_at: i64,
    pub update_at: i64,
}

impl UserRegisterForm {
    pub fn get_auth(&self, user_id: i64) -> UserAuthRegisterForm {
        UserAuthRegisterForm {
            login_type: self.login_type.clone(),
            auth_token: self.auth_token.clone(),
            auth_name: self.auth_name.clone(),
            user_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserAuthRegisterForm {
    pub login_type: LoginType,
    pub auth_token: String,
    pub auth_name: String,
    pub user_id: i64,
}
