use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionType {
    Swap,
    FiatTransfer,
    CryptoTransfer,
}

impl TransactionType {
    pub fn map_tx_type(&self) -> String {
        match self {
            TransactionType::CryptoTransfer => "CryptoTransfer".to_string(),
            TransactionType::FiatTransfer => "FiatTransfer".to_string(),
            TransactionType::Swap => "Swap".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneralTransactionResponseDTO {
    pub status: i32,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub message: String,
    pub data: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneralReadResponseDTO {
    pub status: i32,
    pub message: String,
    pub data: Value,
}

//Fiat transaction
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FiatTransactionRequestDTO {
    pub amount: String,
    pub chain: String,
    pub destination_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FiatTransactionResponseDTO {
    pub pakage: String,
}

//Crypto transaction
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoTransactionRequestDTO {
    pub sender_private_key: String,
    pub recipient_address: String,
    pub amount: String,
    pub chain: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoTransactionResponseDTO {
    pub transaction_hash: String,
}

//Crypto Balance transaction
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoBalanceRequestDTO {
    pub signer_private_key: String,
    pub chain: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoBalanceResponseDTO {
    pub balance: String,
}

//Crypto Wallet
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoWalletRequestDTO {
    pub signer_private_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoWalletResponseDTO {
    pub address: String,
}

//Crypto Wallet creation response
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoWalletCreationResponseDTO {
    pub address: String,
    pub private_key: String,
}

//Crypto Swap
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoSwapRequestDTO {
    pub from_token: String,
    pub to_token: String,
    pub amount: String,
    pub signer_private_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CryptoSwapResponseDTO {
    pub transaction_hash: String,
}
