use crate::helper::{
    get_failed_response, get_success_response, process_failed_response, process_success_response,
};
use app::{
    self,
    usecase::{
        chain_service::ChainService, database_service::DatabaseService,
        payment_service::PaymentService, web3_service::Web3Service,
    },
};
use domain::{
    self,
    shared::dtos::{
        CryptoBalanceRequestDTO, CryptoBalanceResponseDTO, CryptoSwapRequestDTO,
        CryptoTransactionRequestDTO, CryptoWalletCreationResponseDTO, CryptoWalletRequestDTO,
        CryptoWalletResponseDTO, FiatTransactionRequestDTO, ProcessCryptoTransactionRequestDTO,
        TransactionHistoryRequestDTO, TransactionHistoryResponseDTO, TransactionType,
    },
};
use infra::{
    self, celestia_repository::CelestiaRepository, circle_repository::CircleRepository,
    infura_repository::InfuraRepository, postgres_repository::PostgresRepository,
};
use lambda_http::{aws_lambda_events::encodings::Error, Body, Request, Response};
use rocket::serde::json::to_value;
use serde_json::to_string;
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
        Ok(response) => {
            let json_value = to_value(response.clone()).unwrap();
            let final_response = process_success_response(
                json_value,
                TransactionType::FiatTransfer,
                &response.clone().receipient_address,
            )
            .await;
            Ok(final_response)
        }
        Err(error) => Ok(process_failed_response(
            error.to_string(),
            "Failed",
            TransactionType::FiatTransfer,
        )),
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
        Ok(response) => {
            let json_value = to_value(response.clone()).unwrap();
            let final_response = process_success_response(
                json_value,
                TransactionType::CryptoTransfer,
                &response.clone().sender_address,
            )
            .await;
            Ok(final_response)
        }
        Err(error) => Ok(process_failed_response(
            error.to_string(),
            "Failed",
            TransactionType::CryptoTransfer,
        )),
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
        Ok(response) => {
            let rs = CryptoBalanceResponseDTO { balance: response };
            let json_str = to_string(&rs).unwrap();
            Ok(get_success_response(json_str))
        }
        Err(err) => Ok(get_failed_response(err.to_string(), "Failed")),
    }
}

//allow user to get their wallet by their private key
pub async fn crypto_wallet(event: Request) -> Result<Response<Body>, Error> {
    let repository = Arc::new(InfuraRepository::new());
    let web3_service = Web3Service::new(repository);

    let body = event.body();
    let body_str = String::from_utf8(body.as_ref().to_vec())?;
    let crypto_wallet_request: CryptoWalletRequestDTO = serde_json::from_str(&body_str)?;

    match web3_service
        .get_wallet(&crypto_wallet_request.signer_private_key)
        .await
    {
        Ok(response) => {
            let rs = CryptoWalletResponseDTO { address: response };
            let json_str = to_string(&rs).unwrap();
            Ok(get_success_response(json_str))
        }
        Err(err) => Ok(get_failed_response(err.to_string(), "Failed")),
    }
}

//user can create their own wallet
pub async fn crypto_wallet_creation(_: Request) -> Result<Response<Body>, Error> {
    let repository = Arc::new(InfuraRepository::new());
    let web3_service = Web3Service::new(repository);

    match web3_service.create_wallet().await {
        Ok(response) => {
            let rs = CryptoWalletCreationResponseDTO {
                address: response.0,
                private_key: response.1,
            };
            let json_str = to_string(&rs).unwrap();
            Ok(get_success_response(json_str))
        }
        Err(err) => Ok(get_failed_response(err.to_string(), "Failed")),
    }
}

//user can swap their crypto
pub async fn crypto_swap(event: Request) -> Result<Response<Body>, Error> {
    let repository = Arc::new(InfuraRepository::new());
    let web3_service = Web3Service::new(repository);

    let body = event.body();
    let body_str = String::from_utf8(body.as_ref().to_vec())?;
    let crypto_swap_request: CryptoSwapRequestDTO = serde_json::from_str(&body_str)?;

    match web3_service
        .swap(
            &crypto_swap_request.from_token,
            &crypto_swap_request.to_token,
            &crypto_swap_request.amount,
            &crypto_swap_request.signer_private_key,
        )
        .await
    {
        Ok(response) => {
            let json_value = serde_json::to_value(response.clone())?;
            let final_response = process_success_response(
                json_value,
                TransactionType::Swap,
                &response.clone().address,
            )
            .await;
            Ok(final_response)
        }
        Err(err) => Ok(process_failed_response(
            err.to_string(),
            "Failed",
            TransactionType::Swap,
        )),
    }
}

//user can get their transaction history
pub async fn transaction_history(event: Request) -> Result<Response<Body>, Error> {
    let db_repository = Arc::new(PostgresRepository::new().await);
    let db_service = DatabaseService::new(db_repository.clone());

    let body = event.body();
    let body_str = String::from_utf8(body.as_ref().to_vec())?;
    let transaction_history_request: TransactionHistoryRequestDTO =
        serde_json::from_str(&body_str)?;

    let transaction_result = db_service
        .fetch_related_transactions(&transaction_history_request.address.to_lowercase())
        .await?;

    let chain_repository = Arc::new(CelestiaRepository::new().await);
    let chain_service = ChainService::new(chain_repository.clone());

    let mut transaction_from_node = Vec::<TransactionHistoryResponseDTO>::new();
    for tx in transaction_result {
        println!("height : {:?}, address : {:?}", tx.w3_height, tx.w3_address);
        let namespace = chain_repository
            .clone()
            .address_to_namespace(&tx.w3_address)?;
        let blobs = chain_service
            .get_all(&[namespace], tx.w3_height.parse::<u64>().unwrap())
            .await?;
        transaction_from_node.append(&mut blobs.clone());
    }
    let json_str = to_string(&transaction_from_node).unwrap();
    Ok(get_success_response(json_str))
}

//client send the raw transaction for us to process
pub async fn process_crypto_transaction(event: Request) -> Result<Response<Body>, Error> {
    let repository = Arc::new(InfuraRepository::new());
    let web3_service = Web3Service::new(repository);

    let body = event.body();
    let body_str = String::from_utf8(body.as_ref().to_vec())?;
    let process_crypto_transaction_request: ProcessCryptoTransactionRequestDTO =
        serde_json::from_str(&body_str)?;

    match web3_service
        .process_crypto_transaction(&process_crypto_transaction_request.tx_hash)
        .await
    {
        Ok(response) => {
            let json_str = to_string(&response).unwrap();
            Ok(get_success_response(json_str))
        }
        Err(err) => Ok(get_failed_response(err.to_string(), "Failed")),
    }
}
