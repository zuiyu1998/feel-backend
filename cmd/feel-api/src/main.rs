use abi::{tokio, Result};
use api::ApiConfig;
use tools::config_util;

#[tokio::main]
async fn main() -> Result<()> {
    let api = config_util::load_config_with_file_name("config/api_server.yaml")?;
    let log = config_util::load_config_with_file_name("config/log.yaml")?;
    let db = config_util::load_config_with_file_name("config/db.yaml")?;
    let jwt = config_util::load_config_with_file_name("config/jwt.yaml")?;
    let redis = config_util::load_config_with_file_name("config/redis.yaml")?;

    let config = ApiConfig {
        api,
        log,
        db,
        jwt,
        redis,
    };

    if let Err(e) = api::start(&config).await {
        print!("api start error: {}", e);
    }

    Ok(())
}
