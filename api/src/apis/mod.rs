mod user;
mod utils;

use poem_openapi::OpenApi;

pub fn get_apis() -> impl OpenApi {
    return user::UserApi;
}
