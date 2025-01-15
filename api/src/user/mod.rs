use crate::{error::Result, routes::AppState, utils::AppJson};
use axum::{extract::State, routing::post, Json, Router};
use axum_auth::AuthBearer;
use service::user::{UserLoginReq, UserRegisterReq};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/unregister", post(unregister))
}

async fn unregister(State(state): State<AppState>, AuthBearer(token): AuthBearer) -> Result<AppJson<()>> {
    state.user_service.unregister(&token).await?;

    Ok(AppJson::ok(()))
}
async fn register(
    State(state): State<AppState>,
    Json(req): Json<UserRegisterReq>,
) -> Result<AppJson<()>> {
    state.user_service.register(&req).await?;

    Ok(AppJson::ok(()))
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<UserLoginReq>,
) -> Result<AppJson<String>> {
    let token = state.user_service.login(req).await?;

    Ok(AppJson::ok(token))
}
