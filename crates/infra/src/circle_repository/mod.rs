use async_trait::async_trait;
use domain::repository::payment_repository::PaymentRepository;
use request_model::{Amount, Destination, Source, TransferRequest};
use reqwest::{header::CONTENT_TYPE, Client, Error, RequestBuilder, Response};
use serde::Serialize;
use serde_json::{self, to_string, Value};
use std::env;
use uuid::Uuid;
pub mod request_model;

pub struct CircleRepository {
    client: Client,
    mint_base_url: String,
    mint_api_key: String,
}

pub enum RequestMethod {
    POST,
    GET,
}

impl CircleRepository {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            mint_api_key: env::var("CIRCLE_MINT_API_KEY").expect("Circle mint API Key must be set"),
            mint_base_url: env::var("CIRCLE_MINT_BASE_URL").expect("Mint base url must be set"),
        }
    }

    async fn send_request<T: Serialize>(
        &self,
        body: Option<T>,
        endpoint: &String,
        method: RequestMethod,
    ) -> Result<Response, Error> {
        let json = match body {
            Some(b) => to_string(&b).unwrap(),
            None => "".to_string(),
        };
        let mut request: RequestBuilder = match method {
            RequestMethod::GET => self.client.get(endpoint),
            RequestMethod::POST => self.client.post(endpoint),
        };
        request = request
            .header("Authorization", format!("Bearer {}", self.mint_api_key))
            .header(CONTENT_TYPE, "application/json")
            .body(json.clone());

        let response = request
            .send()
            .await
            .map_err(|e| format!("[ERROR] Circle Endpoint: {endpoint} - details: {e}"));

        Ok(response.unwrap())
    }

    async fn get_master_wallet_id(&self) -> Result<Response, Error> {
        let endpoint = format!("{}/v1/configuration", self.mint_base_url);
        let response = self
            .send_request(None::<()>, &endpoint, RequestMethod::GET)
            .await;
        Ok(response.unwrap())
    }
}

#[async_trait]
impl PaymentRepository for CircleRepository {
    async fn process_fiat(
        &self,
        amount: &str,
        chain: &str,
        destination_address: &str,
    ) -> Result<String, String> {
        let endpoint = format!("{}/v1/transfers", self.mint_base_url);
        let json_wallet = self
            .get_master_wallet_id()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let wallet_value: Value = serde_json::from_str(&json_wallet).unwrap();
        let master_wallet_id = wallet_value["data"]["payments"]["masterWalletId"].as_str();

        let payload = TransferRequest {
            source: Source {
                id: master_wallet_id.unwrap_or_default().to_string(),
                source_type: "wallet".to_string(),
            },
            amount: Amount {
                amount: amount.to_string(),
                currency: "USD".to_string(),
            },
            destination: Destination {
                destination_type: "blockchain".to_string(),
                chain: chain.to_string(),
                address: destination_address.to_string(),
            },
            idempotency_key: Uuid::new_v4().to_string(),
        };

        let response = self
            .send_request(Some(payload), &endpoint, RequestMethod::POST)
            .await;
        Ok(response.unwrap().text().await.unwrap())
    }
}
