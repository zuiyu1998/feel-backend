use poem_openapi::{auth::Bearer, SecurityScheme};

use poem::Request;

use crate::state::AppState;

#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    key_name = "Authorization",
    key_in = "header",
    checker = "api_checker"
)]
pub struct Authorization(pub i32);

async fn api_checker(req: &Request, bearer: Bearer) -> Option<i32> {
    let state = req.data::<AppState>();

    if state.is_none() {
        return None;
    }

    let state = state.unwrap();

    state
        .jwt_helper
        .decode(&bearer.token)
        .ok()
        .and_then(|sub| sub.parse::<i32>().ok())
}
