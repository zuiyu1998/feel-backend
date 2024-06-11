mod error;

mod apis;
mod state;

pub use error::*;

use abi::{
    config::{Config, FromConfig},
    tracing,
};
use apis::get_apis;

use poem::{listener::TcpListener, middleware::Tracing, Endpoint, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use state::AppState;

pub fn app(config: &Config) -> impl Endpoint {
    let api_service =
        OpenApiService::new(get_apis(), &config.poem.api_title, &config.poem.api_version)
            .server(config.poem.get_url_prefix());
    let ui = api_service.swagger_ui();

    Route::new()
        .nest(config.poem.get_url_prefix(), api_service)
        .nest("/", ui)
        .catch_error(|e| handle_error(e))
}

pub async fn start_server(config: &Config) -> Result<()> {
    let state = AppState::from_config(config).await?;

    let app = app(&config).with(Tracing::default()).data(state);

    let bind_addr = config.poem.get_bind_addr();

    tracing::info!("server listening: {}", bind_addr);

    Server::new(TcpListener::bind(&bind_addr)).run(app).await?;

    Ok(())
}
