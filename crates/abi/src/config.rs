use tracing::Level;

#[derive(Debug, Default)]
pub struct Config {
    pub api: ApiConfig,
    pub log: LogConfig,
    pub sha: ShaConfig,
    pub db: DbConfig,
    pub jwt: JWTConfig,
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            database_url: "postgresql://postgres:bj123456@localhost/feel".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JWTConfig {
    pub security: String,
    pub issuer: String,
}

impl Default for JWTConfig {
    fn default() -> Self {
        JWTConfig {
            security: "feel-jwt".to_string(),
            issuer: "feel.com".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShaConfig {
    pub security: String,
}

impl Default for ShaConfig {
    fn default() -> Self {
        ShaConfig {
            security: "feel-sha2".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]

pub enum TraceLevel {
    #[default]
    Debug,
}

impl TraceLevel {
    pub fn level(&self) -> Level {
        match self {
            TraceLevel::Debug => Level::DEBUG,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LogConfig {
    pub level: TraceLevel,
    pub filter: String,
}

#[derive(Debug)]
pub struct ApiConfig {
    host: String,
    port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

impl ApiConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
