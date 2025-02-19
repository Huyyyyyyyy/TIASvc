use anyhow::{Ok, Result};
use async_trait::async_trait;
use celestia_rpc::prelude::*;
use celestia_rpc::{Client, TxConfig};
use celestia_types::nmt::Namespace;
use celestia_types::Blob;
use domain::repository::chain_repository::ChainRepository;
use std::sync::OnceLock;
use tokio::sync::{Mutex, MutexGuard};

pub struct CelestiaRepository {
    pub client: Client,
    pub url: String,
    pub auth_token: String,
}

impl CelestiaRepository {
    pub async fn new() -> Self {
        let url = std::env::var("CELESTIA_RPC_URL").expect("Celestia rpc url must be set");
        let auth_token =
            std::env::var("CELESTIA_AUTH_TOKEN").expect("Celestia auth token must be set");
        let client = Client::new(&url, Some(&auth_token)).await.unwrap();
        client.header_wait_for_height(2).await.unwrap();
        Self {
            client,
            url,
            auth_token,
        }
    }

    pub async fn write_lock(&self) -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().await
    }
}

#[async_trait]
impl ChainRepository<Blob, Namespace> for CelestiaRepository {
    async fn submit(&self, blobs: &[Blob]) -> Result<u64> {
        let _guard = self.write_lock().await;
        let height = self.client.blob_submit(blobs, TxConfig::default()).await?;
        Ok(height)
    }

    async fn get_all(&self, namespace: &[Namespace], height: u64) -> Result<()> {
        let blobs = self.client.blob_get_all(height, namespace).await?;
        match blobs {
            Some(blobs) => {
                for blob in blobs {
                    println!("found at height {:?}", height);
                }
            }
            None => {}
        }
        Ok(())
    }
}
