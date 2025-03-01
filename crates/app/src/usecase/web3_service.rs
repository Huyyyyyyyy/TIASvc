use std::sync::Arc;

use anyhow::Result;
use domain::{
    repository::web3_repository::Web3Repository,
    shared::dtos::{CryptoSwapResponseDTO, CryptoTransactionResponseDTO},
};

pub struct Web3Service {
    repository: Arc<dyn Web3Repository>,
}

impl Web3Service {
    pub fn new(repository: Arc<dyn Web3Repository>) -> Self {
        Self { repository }
    }

    pub async fn transfer_token(
        &self,
        sender_private_key: &str,
        recipient_address: &str,
        amount: &str,
        chain: &str,
    ) -> Result<CryptoTransactionResponseDTO> {
        self.repository
            .transfer_token(sender_private_key, recipient_address, amount, chain)
            .await
    }

    pub async fn get_balance(&self, signer_private_key: &str, chain: &str) -> Result<String> {
        self.repository.get_balance(signer_private_key, chain).await
    }

    pub async fn get_wallet(&self, signer_private_key: &str) -> Result<String> {
        self.repository.get_wallet(signer_private_key).await
    }

    pub async fn create_wallet(&self) -> Result<(String, String)> {
        self.repository.create_wallet().await
    }

    pub async fn swap(
        &self,
        from_token: &str,
        to_token: &str,
        amount: &str,
        signer_private_key: &str,
    ) -> Result<CryptoSwapResponseDTO> {
        self.repository
            .swap(from_token, to_token, amount, signer_private_key)
            .await
    }
}
