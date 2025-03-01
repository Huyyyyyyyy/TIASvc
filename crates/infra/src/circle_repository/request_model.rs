use serde::{self, Deserialize, Serialize};

//General Struct
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Amount {
    pub amount: String,
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Destination {
    #[serde(rename = "type")]
    pub destination_type: String,
    pub address: String,
    pub chain: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PaymentMethods {
    #[serde(rename = "type")]
    pub method_type: String,
    pub chain: String,
}

//Transfer
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TransferRequest {
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: String,
    pub source: Source,
    pub amount: Amount,
    pub destination: Destination,
}