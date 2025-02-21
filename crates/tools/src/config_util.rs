use abi::{
    config::{self, Config},
    Result,
};
use serde::de::DeserializeOwned;

pub fn load_config_with_file_name<T: DeserializeOwned>(path: &str) -> Result<T> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .build()?;

    let value = config.try_deserialize::<T>()?;

    Ok(value)
}
