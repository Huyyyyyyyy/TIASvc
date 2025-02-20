use crate::shared::dtos::FiatTransactionResponseDTO;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn process_fiat(
        &self,
        amount: &str,
        chain: &str,
        destination_address: &str,
    ) -> Result<FiatTransactionResponseDTO>;
}
