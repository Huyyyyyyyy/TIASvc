use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Web3Repository: Send + Sync {
    async fn transfer_erc20_token(
        &self,
        sender_private_key: &str,
        recipient_address: &str,
        amount: &str,
        chain: &str,
    ) -> Result<String>;
}
