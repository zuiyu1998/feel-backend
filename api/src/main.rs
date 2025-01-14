use abi::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::default();

    if let Err(e) = api::start(&config).await {
        print!("api start error: {}", e);
    }
}
