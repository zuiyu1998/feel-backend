use abi::{protocol::pb::feel_sdk::RegisterUserResp, tonic::async_trait, tracing, Result};

pub struct WebHookServiceCommon;

#[async_trait]
impl WebHookService for WebHookServiceCommon {
    async fn register_chat_user(&self, _user: &RegisterUserResp) -> Result<()> {
        tracing::info!("register_chat_user start");

        Ok(())
    }
}

#[async_trait]
pub trait WebHookService: 'static + Sync + Send {
    async fn register_chat_user(&self, user: &RegisterUserResp) -> Result<()>;
}
