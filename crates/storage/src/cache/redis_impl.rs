use crate::user::dto::*;
use abi::{
    redis::{self, AsyncCommands},
    tonic::async_trait,
    tracing::info,
    Result,
};
use serde::{Deserialize, Serialize};
use tools::json_util;

use super::Cache;

pub const USER_BASE_KEY: &str = "__user_base_key";
pub const USER_AUTH_KEY: &str = "__user_auth_key";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
}

impl RedisConfig {
    pub fn url(&self) -> String {
        format!("redis://{}@{}:{}", self.password, self.host, self.port)
    }
}

pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn from_config(config: &RedisConfig) -> Result<Self> {
        info!("Redis connect {}", config.url());

        let client = redis::Client::open(config.url())?;

        Ok(RedisCache { client })
    }
}

#[async_trait]
impl Cache for RedisCache {
    //获取用户信息
    async fn get_user_base(&self, id: i64) -> Result<Option<User>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let raw_value: Option<String> = conn.hget(USER_BASE_KEY, id.to_string()).await?;

        if let Some(raw_value) = raw_value {
            let user: User = json_util::parse(&raw_value)?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    //插入用户信息
    async fn insert_user_base(&self, id: i64, user: User) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value = json_util::to_string(&user);
        let _: () = conn.hset(USER_BASE_KEY, id.to_string(), value).await?;

        Ok(())
    }

    //获取用户授权
    async fn get_user_auth(
        &self,
        auth_type: UserAuthType,
        auth_name: &str,
    ) -> Result<Option<UserAuth>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let field = format!("{}-{}", auth_type.as_str(), auth_name);

        let raw_value: Option<String> = conn.hget(USER_AUTH_KEY, field).await?;

        if let Some(raw_value) = raw_value {
            let auth: UserAuth = json_util::parse(&raw_value)?;
            Ok(Some(auth))
        } else {
            Ok(None)
        }
    }

    //插入用户授权
    async fn insert_user_auth(
        &self,
        auth_type: UserAuthType,
        auth_name: &str,
        user_auth: UserAuth,
    ) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let field = format!("{}-{}", auth_type.as_str(), auth_name);

        let value = json_util::to_string(&user_auth);
        let _: () = conn.hset(USER_AUTH_KEY, field, value).await?;

        Ok(())
    }
}

mod test {
    #[test]
    fn test_redis_cache() {
        assert_eq!(true, true);
    }
}
