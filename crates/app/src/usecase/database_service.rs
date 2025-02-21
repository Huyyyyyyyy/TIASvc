use anyhow::Result;
use domain::{entity::W3Transaction, repository::database_repository::DatabaseRepository};
use std::sync::Arc;

pub struct DatabaseService {
    pub repository: Arc<dyn DatabaseRepository>,
}

impl DatabaseService {
    pub fn new(repository: Arc<dyn DatabaseRepository + Send + Sync>) -> Self {
        Self { repository }
    }
    pub async fn fetch_related_transactions(&self, address: &str) -> Result<Vec<W3Transaction>> {
        self.repository.get_related_transaction(address).await
    }

    pub async fn add_new_transaction(&self, height: &str, address: &str) -> Result<()> {
        self.repository
            .insert_new_transaction(height, address)
            .await
    }
}
