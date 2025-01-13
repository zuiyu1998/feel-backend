use tracing::Level;

#[derive(Debug, Default)]
pub struct Config {
    pub api: ApiConfig,
    pub log: LogConfig,
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
