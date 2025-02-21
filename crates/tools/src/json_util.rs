use abi::{serde_json, Result};
use serde::{de::DeserializeOwned, Serialize};

pub fn to_string<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap()
}

pub fn parse<T: DeserializeOwned>(raw: &str) -> Result<T> {
    let value = serde_json::from_str(raw)?;
    Ok(value)
}
