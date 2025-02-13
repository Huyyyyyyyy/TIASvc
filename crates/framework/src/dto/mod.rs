use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneralResponseDTO {
    pub status: String,
    pub message: String,
    pub data: String,
}
//Fiat transaction
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FiatTransactionRequestDTO {
    pub amount: String,
    pub chain: String,
    pub destination_address: String,
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
