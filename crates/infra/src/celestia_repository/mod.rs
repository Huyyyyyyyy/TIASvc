use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use celestia_rpc::prelude::*;
use celestia_rpc::{Client, TxConfig};
use celestia_types::nmt::Namespace;
use celestia_types::{AppVersion, Blob};
use domain::repository::chain_repository::ChainRepository;
use serde_json::Value;
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

    pub fn address_to_namespace(&self, address: &str) -> Result<[u8; 8]> {
        // Remove "0x" if present.
        let addr = address.strip_prefix("0x").unwrap_or(address);
        // Decode the hex string into raw bytes.
        let decoded = BASE64_STANDARD.decode(addr)?;
        if decoded.len() < 8 {
            return Err(anyhow!("Address decoded bytes less than 8"));
        }
        // Take the first 8 bytes.
        let mut ns = [0u8; 8];
        ns.copy_from_slice(&decoded[..8]);
        Ok(ns)
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

    async fn build_blob(&self, namespace: &str, data: Value) -> Result<Blob> {
        let user_namespace_bytes: [u8; 8] = self.address_to_namespace(namespace)?;
        let namespace: Namespace = Namespace::new_v0(&user_namespace_bytes)?;
        let data_str = serde_json::to_string(&data)?;
        let encoded_data: Vec<u8> = BASE64_STANDARD.encode(data_str).as_bytes().to_vec();
        let blob: Blob = Blob::new(namespace, encoded_data, AppVersion::V2)?;
        Ok(blob)
    }
}
