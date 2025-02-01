use domain::repository::payment_repository::PaymentRepository;
use std::sync::Arc;

pub struct PaymentService {
    repository: Arc<dyn PaymentRepository>,
}

impl PaymentService {
    pub fn new(repository: Arc<dyn PaymentRepository>) -> Self {
        Self { repository }
    }

    pub async fn transfers(
        &self,
        amount: &str,
        chain: &str,
        destination_address: &str,
    ) -> Result<String, String> {
        self.repository
            .transfers(amount, chain, destination_address)
            .await
    }
}
