use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TranfersRequestDTO {
    pub amount: String,
    pub chain: String,
    pub destination_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TransfersResponseDTO {
    pub status: String,
    pub message: String,
    pub data: String,
}
