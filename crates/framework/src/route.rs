use crate::{
    dto::{CryptoBalanceRequestDTO, CryptoTransactionRequestDTO, FiatTransactionRequestDTO},
    helper::{get_failed_response, get_success_response},
};
use app::{
    self,
    usecase::{payment_service::PaymentService, web3_service::Web3Service},
};
use infra::{self, circle_repository::CircleRepository, infura_repository::InfuraRepository};
use lambda_http::{aws_lambda_events::encodings::Error, Body, Request, Response};
use std::sync::Arc;

//transaction to transfer fiat to users wallet after banking payment
pub async fn fiat_transaction(event: Request) -> Result<Response<Body>, Error> {
    let repository = Arc::new(CircleRepository::new());
    let payment_service = PaymentService::new(repository);

    let body = event.body();
    let body_str = String::from_utf8(body.as_ref().to_vec())?;
    let fiat_transaction_request: FiatTransactionRequestDTO = serde_json::from_str(&body_str)?;

    match payment_service
        .process_fiat(
            &fiat_transaction_request.amount,
            &fiat_transaction_request.chain,
            &fiat_transaction_request.destination_address,
        )
        .await
    {
        Ok(response) => Ok(get_success_response(response)),
        Err(error) => Ok(get_failed_response(error)),
    }
}

//transaction to transfer erc20 token between users
pub async fn crypto_transaction(event: Request) -> Result<Response<Body>, Error> {
    let repository = Arc::new(InfuraRepository::new());
    let web3_service = Web3Service::new(repository);

    let body = event.body();
    let body_str = String::from_utf8(body.as_ref().to_vec())?;
    let crypto_transaction_request: CryptoTransactionRequestDTO = serde_json::from_str(&body_str)?;

    match web3_service
        .transfer_token(
            &crypto_transaction_request.sender_private_key,
            &crypto_transaction_request.recipient_address,
            &crypto_transaction_request.amount,
            &crypto_transaction_request.chain,
        )
        .await
    {
        Ok(response) => Ok(get_success_response(response)),
        Err(error) => Ok(get_failed_response(error.to_string())),
    }
}

//allow client get their balance base on chain
pub async fn crypto_balance(event: Request) -> Result<Response<Body>, Error> {
    let repository = Arc::new(InfuraRepository::new());
    let web3_service = Web3Service::new(repository);

    let body = event.body();
    let body_str = String::from_utf8(body.as_ref().to_vec())?;
    let crypto_balance_request: CryptoBalanceRequestDTO = serde_json::from_str(&body_str)?;

    match web3_service
        .get_balance(
            &crypto_balance_request.signer_private_key,
            &crypto_balance_request.chain,
        )
        .await
    {
        Ok(response) => Ok(get_success_response(response)),
        Err(err) => Ok(get_failed_response(err.to_string())),
    }
}
