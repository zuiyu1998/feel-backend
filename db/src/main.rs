use abi::tokio;

use db::{start_server, Result};

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await?;
    Ok(())
}
