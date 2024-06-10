mod error;

mod apis;
mod state;

pub use error::*;

use abi::{config::Config, tracing, tracing_subscriber};
use apis::get_apis;

use poem::{listener::TcpListener, Endpoint, Route, Server};
use poem_openapi::OpenApiService;

pub fn app(config: &Config) -> impl Endpoint {
    let api_service =
        OpenApiService::new(get_apis(), &config.poem.api_title, &config.poem.api_version)
            .server(config.poem.get_url_prefix());
    let ui = api_service.swagger_ui();

    Route::new()
        .nest(config.poem.get_url_prefix(), api_service)
        .nest("/", ui)
}

pub async fn start_server() -> Result<()> {
    let config = Config::load()?;
    tracing_subscriber::fmt::init();

    let app = app(&config);

    let bind_addr = config.poem.get_bind_addr();

    tracing::info!("server listening: {}", bind_addr);

    Server::new(TcpListener::bind(&bind_addr)).run(app).await?;

    Ok(())
}
