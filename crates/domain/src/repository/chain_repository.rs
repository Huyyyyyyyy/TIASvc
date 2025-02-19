use anyhow::Result;
use async_trait::async_trait;

// Define a trait for submitting blobs.
// This trait is object‑safe because it only uses &self.
#[async_trait]
pub trait ChainRepository<B, C> {
    async fn submit(&self, blobs: &[B]) -> Result<u64>;

    async fn get_all(&self, namespace: &[C], height: u64) -> Result<()>;
}
