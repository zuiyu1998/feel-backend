use abi::{config::Config, log::logger_init};

#[tokio::main]
async fn main() {
    let config = Config::default();

    logger_init(&config.log);

    if let Err(e) = api::start(&config.api).await {
        print!("api start error: {}", e);
    }
}
