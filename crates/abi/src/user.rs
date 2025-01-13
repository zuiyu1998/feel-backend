pub struct User {
    id: i64,
    uid: String,
    nikename: String,
    create_at: i64,
    update_at: i64,
}

///登录类型
pub enum LoginType {
    ///手机
    Phone,
}

pub struct UserLoginForm {
    login_type: LoginType,
    auth_token: String,
    auth_name: String,
}

pub struct UserRegisterForm {
    login_type: LoginType,
    auth_token: String,
    auth_name: String,
    nikename: String,
}

pub struct UserUnregisterForm {
    uid: String,
    token: String,
}
