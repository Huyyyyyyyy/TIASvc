use crate::dto::{CryptoTransactionRequestDTO, FiatTransactionRequestDTO, GeneralResponseDTO};
use app::{
    self,
    usecase::{payment_service::PaymentService, web3_service::Web3Service},
};
use infra::{self, circle_repository::CircleRepository, infura_repository::InfuraRepository};
use rocket::serde::json::Json;
use std::sync::Arc;

#[post(
    "/fiat-transaction",
    format = "application/json",
    data = "<fiat_transaction_request>"
)]
pub async fn fiat_transaction(
    fiat_transaction_request: Json<FiatTransactionRequestDTO>,
) -> Json<GeneralResponseDTO> {
    let repository = Arc::new(CircleRepository::new());
    let payment_service = PaymentService::new(repository);

    match payment_service
        .process_fiat(
            &fiat_transaction_request.amount,
            &fiat_transaction_request.chain,
            &fiat_transaction_request.destination_address,
        )
        .await
    {
        Ok(response) => Json(GeneralResponseDTO {
            status: "200".to_string(),
            message: "success".to_string(),
            data: response,
        }),
        Err(error) => Json(GeneralResponseDTO {
            status: "400".to_string(),
            message: error,
            data: "".to_string(),
        }),
    }
}

#[post(
    "/crypto-transaction",
    format = "application/json",
    data = "<crypto_transaction_request>"
)]
pub async fn crypto_transaction(
    crypto_transaction_request: Json<CryptoTransactionRequestDTO>,
) -> Json<GeneralResponseDTO> {
    let repository = Arc::new(InfuraRepository::new());
    let web3_service = Web3Service::new(repository);

    match web3_service
        .transfer_erc20_token(
            &crypto_transaction_request.sender_private_key,
            &crypto_transaction_request.recipient_address,
            &crypto_transaction_request.amount,
            &crypto_transaction_request.chain,
        )
        .await
    {
        Ok(response) => Json(GeneralResponseDTO {
            status: "200".to_string(),
            message: "success".to_string(),
            data: response,
        }),
        Err(error) => Json(GeneralResponseDTO {
            status: "400".to_string(),
            message: error.to_string(),
            data: "".to_string(),
        }),
    }
}
