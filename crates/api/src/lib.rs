pub mod routes;
mod user;

mod utils;

mod error;

use abi::{log, tokio, tracing, Result};

pub use routes::{ApiConfig, AppState};

pub async fn start(config: &ApiConfig) -> Result<()> {
    log::logger_init(&config.log);

    let app_state = AppState::from_config(config).await?;

    let app = routes::routes().with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.api.addr()).await?;

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}
