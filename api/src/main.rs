use abi::tokio;

use api::{start_server, Result};

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await?;
    Ok(())
}
