use anyhow::Result;
use domain::{
    repository::chain_repository::ChainRepository, shared::dtos::TransactionHistoryResponseDTO,
};
use serde_json::Value;
use std::sync::Arc;

pub struct ChainService<B, C> {
    pub repository: Arc<dyn ChainRepository<B, C> + Send + Sync>,
}

impl<B, C> ChainService<B, C> {
    pub fn new(repository: Arc<dyn ChainRepository<B, C> + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn submit(&self, blobs: &[B]) -> Result<u64> {
        self.repository.submit(blobs).await
    }

    pub async fn build_blob(&self, namespace: &str, data: Value) -> Result<B> {
        self.repository.build_blob(namespace, data).await
    }

    pub async fn get_all(
        &self,
        namespace: &[C],
        height: u64,
    ) -> Result<Vec<TransactionHistoryResponseDTO>> {
        self.repository.get_all(namespace, height).await
    }

    fn revert_blob(&self, blob: &B) -> Result<TransactionHistoryResponseDTO> {
        self.repository.revert_blob(blob)
    }
}
