use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JWTConfig {
    pub security: String,
    pub issuer: String,
}

pub struct JwtHelper {
    security: String,
    issuer: String,
}

impl JwtHelper {
    pub fn from_config(config: &JWTConfig) -> Self {
        JwtHelper {
            security: config.security.clone(),
            issuer: config.issuer.clone(),
        }
    }

    pub fn encode(&self, uid: &str) -> String {
        let claims = RegisteredClaims {
            issuer: Some(self.issuer.clone()),
            subject: Some(uid.to_string()),
            ..Default::default()
        };

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.security.as_bytes()).unwrap();
        claims.sign_with_key(&key).unwrap()
    }

    pub fn decode(&self, token: &str) -> Option<String> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.security.as_bytes()).unwrap();
        let claims: RegisteredClaims = VerifyWithKey::verify_with_key(token, &key).ok()?;

        let sub = claims.subject?;

        Some(sub)
    }
}
