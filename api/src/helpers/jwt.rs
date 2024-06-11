use crate::{Error, Result};
use abi::{
    chrono::{Duration, Local},
    config::{Config, FromConfig},
    tonic::async_trait,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub struct JwtHelper {
    secret: String,
    days: i32,
}

#[async_trait]
impl FromConfig for JwtHelper {
    type Error = Error;

    async fn from_config(config: &Config) -> Result<Self, Self::Error> {
        Ok(JwtHelper {
            secret: config.poem.jwt.secret.to_string(),
            days: config.poem.jwt.days,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

impl Claims {
    pub fn new(sub: &str, exp: i64, iat: i64) -> Self {
        Claims {
            sub: sub.to_string(),
            exp,
            iat,
        }
    }
}

impl JwtHelper {
    pub fn encode(&self, sub: &str) -> Result<String> {
        let now = Local::now();

        let iat = now.timestamp();
        let exp = (now + Duration::days(self.days as i64)).timestamp();

        let claims = Claims::new(sub, iat, exp);

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn decode(&self, token: &str) -> Result<String> {
        let token_data = jsonwebtoken::decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(token_data.claims.sub)
    }
}
