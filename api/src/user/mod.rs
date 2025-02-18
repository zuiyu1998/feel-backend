use crate::{error::Result, routes::AppState, utils::AppJson};
use abi::protocol::pb::feel_sdk::{RegisterUserReq, UserLoginReq};
use axum::{extract::State, routing::{get, post}, Json, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        // .route("/login", post(login))
        // .route("/unregister", post(unregister))
}

// async fn unregister(State(state): State<AppState>, AuthBearer(token): AuthBearer) -> Result<AppJson<()>> {
//     todo!()
//     // state.user_service.unregister_user(&token).await?;

//     // Ok(AppJson::ok(()))
// }
async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterUserReq>,
) -> Result<AppJson<()>> {
    state.user_service.register_user(req).await?;

    Ok(AppJson::ok(()))
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<UserLoginReq>,
) -> Result<AppJson<String>> {

    todo!()
    // let token = state.user_service.login(req).await?;

    // Ok(AppJson::ok(token))
}
