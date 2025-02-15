use std::sync::Arc;

use anyhow::Result;
use domain::repository::web3_repository::Web3Repository;

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
    ) -> Result<String> {
        self.repository
            .transfer_token(sender_private_key, recipient_address, amount, chain)
            .await
    }

    pub async fn get_balance(&self, signer_private_key: &str, chain: &str) -> Result<String> {
        self.repository.get_balance(signer_private_key, chain).await
    }
}
