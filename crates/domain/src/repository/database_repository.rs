use crate::entity::W3Transaction;
use anyhow::Result;
use rocket::async_trait;
#[async_trait]
pub trait DatabaseRepository: Send + Sync {
    async fn get_related_transaction(&self, address: &str) -> Result<Vec<W3Transaction>>;

    async fn insert_new_transaction(&self, height: &str, address: &str) -> Result<()>;
}
