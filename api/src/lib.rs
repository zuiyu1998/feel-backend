use abi::{config::ApiConfig, Result};
use axum::{response::Html, routing::get, Router};

pub async fn start(config: &ApiConfig) -> Result<()> {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind(&config.addr()).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
