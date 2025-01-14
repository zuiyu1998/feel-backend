use abi::config::JWTConfig;
use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use sha2::Sha256;

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

    pub fn encode(&self, user_id: i64) -> String {
        let claims = RegisteredClaims {
            issuer: Some(self.issuer.clone()),
            subject: Some(user_id.to_string()),
            ..Default::default()
        };

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.security.as_bytes()).unwrap();

        let signed_token = claims.sign_with_key(&key).unwrap();

        signed_token
    }

    pub fn decode(&self, token: &str) -> Option<i64> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.security.as_bytes()).unwrap();
        let claims: RegisteredClaims = VerifyWithKey::verify_with_key(token, &key).ok()?;

        let sub = claims.subject?;

        sub.parse::<i64>().ok()
    }
}
