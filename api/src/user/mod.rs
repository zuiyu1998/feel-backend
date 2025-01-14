use crate::{error::Result, routes::AppState, utils::AppJson};
use axum::{extract::State, routing::post, Json, Router};
use service::user::UserRegisterReq;

pub fn routes() -> Router<AppState> {
    Router::new().route("/register", post(register))
}

async fn register(
    State(state): State<AppState>,
    Json(req): Json<UserRegisterReq>,
) -> Result<AppJson<()>> {
    state.user_service.register(&req).await?;

    Ok(AppJson::ok(()))
}
