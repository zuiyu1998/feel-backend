mod label;
mod user;

pub mod utils;

use poem_openapi::{OpenApi, Tags};

#[derive(Tags)]
pub enum ApiTags {
    ///用户服务
    UserApi,
    ///标签服务
    LabelApi,
}

pub fn get_apis() -> impl OpenApi {
    return (user::UserApi, label::LabelApi);
}
