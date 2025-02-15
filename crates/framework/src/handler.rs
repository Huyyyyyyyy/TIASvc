use crate::route::{
    crypto_balance, crypto_transaction, crypto_wallet, crypto_wallet_creation, fiat_transaction,
};
use lambda_http::{Body, Error, IntoResponse, Request, Response};

pub async fn lambda_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let path = event.uri().path();
    let method = event.method().as_str();
    match (method, path) {
        //GET
        ("GET", "/crypto/balance") => crypto_balance(event).await,
        ("GET", "/crypto/wallet") => crypto_wallet(event).await,

        //POST
        ("POST", "/fiat/transaction") => fiat_transaction(event).await,
        ("POST", "/crypto/transaction") => crypto_transaction(event).await,
        ("POST", "/crypto/wallet") => crypto_wallet_creation(event).await,

        //Out of scope
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::Text("Not found".to_string()))
            .unwrap()),
    }
}
