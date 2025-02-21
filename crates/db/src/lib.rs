pub mod cache;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbConfig {
    pub database_url: String,
}
