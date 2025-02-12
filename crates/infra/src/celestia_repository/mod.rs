use celestia_rpc::prelude::*;
use celestia_rpc::{Client, TxConfig};
use celestia_types::Blob;
use jsonrpsee::core::{client::SubscriptionClientT, ClientError};
use std::env;

pub struct CelestiaRepository {
    pub client: Client,
    pub url: String,
    pub auth_token: String,
}

impl CelestiaRepository {
    pub async fn new() -> Self {
        let url = env::var("CELESTIA_RPC_URL").expect("Celestia rpc url must be set");
        let auth_token = env::var("CELESTIA_AUTH_TOKEN").expect("Celestia auth token must be set");
        let client = Client::new(&url, Some(&auth_token)).await.unwrap();
        Self {
            client,
            url,
            auth_token,
        }
    }

    pub async fn blob_submit<T>(&self, client: &T, blobs: &[Blob]) -> Result<u64, ClientError>
    where
        T: SubscriptionClientT + Sync,
    {
        client.blob_submit(blobs, TxConfig::default()).await
    }
}
