use anyhow::Result;
use domain::{
    repository::payment_repository::PaymentRepository, shared::dtos::FiatTransactionResponseDTO,
};
use std::sync::Arc;

pub struct PaymentService {
    repository: Arc<dyn PaymentRepository>,
}

impl PaymentService {
    pub fn new(repository: Arc<dyn PaymentRepository>) -> Self {
        Self { repository }
    }

    pub async fn process_fiat(
        &self,
        amount: &str,
        chain: &str,
        destination_address: &str,
    ) -> Result<FiatTransactionResponseDTO> {
        self.repository
            .process_fiat(amount, chain, destination_address)
            .await
    }
}
