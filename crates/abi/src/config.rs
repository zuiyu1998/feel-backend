use tracing::Level;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
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
