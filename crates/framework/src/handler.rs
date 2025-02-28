use crate::route::{
    crypto_balance, crypto_swap, crypto_transaction, crypto_wallet, crypto_wallet_creation,
    fiat_transaction, process_crypto_swap, process_crypto_transaction, transaction_history,
};
use lambda_http::{Body, Error, IntoResponse, Request, Response};

pub async fn lambda_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let path = event.uri().path();
    let method = event.method().as_str();
    match (method, path) {
        //POST
        ("POST", "/crypto/balance") => crypto_balance(event).await,
        ("POST", "/crypto/wallet") => crypto_wallet(event).await,
        ("POST", "/history/transaction") => transaction_history(event).await,
        ("POST", "/fiat/transaction") => fiat_transaction(event).await,
        ("POST", "/crypto/transaction") => crypto_transaction(event).await,
        ("POST", "/crypto/creation/wallet") => crypto_wallet_creation(event).await,
        ("POST", "/crypto/swap") => crypto_swap(event).await,
        ("POST", "/crypto/process") => process_crypto_transaction(event).await,
        ("POST", "/crypto/swapProcess") => process_crypto_swap(event).await,

        //Out of scope
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::Text("Not found".to_string()))
            .unwrap()),
    }
}
