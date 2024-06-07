use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub poem: PoemConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PoemConfig {
    pub api_title: String,
    pub api_version: String,
    pub host: String,
    pub port: u16,
    pub api_prefix: String,
    pub api_prefix_version: String,
}

impl Default for PoemConfig {
    fn default() -> Self {
        PoemConfig {
            api_title: "feel-backend".to_owned(),
            api_version: "1.0.0".to_owned(),
            host: "0.0.0.0".to_owned(),
            port: 3000,
            api_prefix: "api".to_owned(),
            api_prefix_version: "v1".to_owned(),
        }
    }
}

impl PoemConfig {
    pub fn get_url_prefix(&self) -> String {
        format!("/{}/{}", self.api_prefix, self.api_prefix_version)
    }

    pub fn get_bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
