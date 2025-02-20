use anyhow::Result;
use async_trait::async_trait;

use crate::shared::dtos::{CryptoSwapResponseDTO, CryptoTransactionResponseDTO};

#[async_trait]
pub trait Web3Repository: Send + Sync {
    async fn transfer_token(
        &self,
        sender_private_key: &str,
        recipient_address: &str,
        amount: &str,
        chain: &str,
    ) -> Result<CryptoTransactionResponseDTO>;

    async fn get_balance(&self, signer_private_key: &str, chain: &str) -> Result<String>;

    async fn get_wallet(&self, signer_private_key: &str) -> Result<String>;

    async fn create_wallet(&self) -> Result<(String, String)>;

    async fn swap(
        &self,
        from_token: &str,
        to_token: &str,
        amount: &str,
        signer_private_key: &str,
    ) -> Result<CryptoSwapResponseDTO>;
}
