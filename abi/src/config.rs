use crate::Result;
use figment::{
    providers::{Env, Format, Serialized, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};
use tonic::async_trait;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EnvConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl EnvConfig {
    fn load() -> Result<Self> {
        dotenvy::dotenv().ok();

        let config: EnvConfig = Figment::new().merge(Env::prefixed("")).extract()?;

        Ok(config)
    }
}

impl Config {
    pub const DEFAYLT_PATH: &'static str = "./config.yaml";

    pub fn load() -> Result<Self> {
        let env = EnvConfig::load()?;
        let mut config: Config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Yaml::file(Self::DEFAYLT_PATH))
            .extract()?;

        config.poem.host = env.host;
        config.poem.port = env.port;

        config.db.postgres = env.database_url;

        Ok(config)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub poem: PoemConfig,
    pub db: DbConfig,
    pub service_center: ServiceCenterConfig,
    pub rpc: RpcConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbConfig {
    pub postgres: String,
    pub secure: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            postgres: "".to_string(),
            secure: "secure".to_string(),
        }
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        RpcConfig {
            health_check: false,
            db: RpcServerConfig {
                protocol: "http".to_owned(),
                port: 50004,
                host: "127.0.0.1".to_owned(),
                name: "db".to_owned(),
                tags: vec!["db".to_owned(), "grpc".to_owned()],
                grpc_health_check: GrpcHealthCheck {
                    grpc_use_tls: false,
                    interval: 3000,
                },
            },
        }
    }
}

#[async_trait]
pub trait FromConfig: Sized {
    type Error;

    async fn from_config(config: &Config) -> Result<Self, Self::Error>;
}

//注册中心配置
#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct ServiceCenterConfig {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub timeout: u64,
}

impl Default for ServiceCenterConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 8500,
            protocol: "http".to_owned(),
            timeout: 5000,
        }
    }
}

pub enum ServiceType {
    Db,
    All,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcConfig {
    pub health_check: bool,
    pub db: RpcServerConfig,
    // pub pusher: RpcServerConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcServerConfig {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub tags: Vec<String>,
    pub grpc_health_check: GrpcHealthCheck,
}

impl RpcServerConfig {
    #[inline]
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[inline]
    pub fn url(&self, https: bool) -> String {
        url(https, &self.host, self.port)
    }
}

fn url(https: bool, host: &str, port: u16) -> String {
    if https {
        format!("https://{}:{}", host, port)
    } else {
        format!("http://{}:{}", host, port)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrpcHealthCheck {
    pub grpc_use_tls: bool,
    pub interval: u16,
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
