use abi::{config::Config, tokio, tracing::Level, tracing_subscriber};

use api::{start_server, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    start_server(&config).await?;
    Ok(())
}
