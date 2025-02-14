use dotenv::{dotenv, from_filename};
use framework::handlers::{crypto_transaction, fiat_transaction};
use lambda_http::{
    aws_lambda_events::encodings::Error, run, service_fn, Body, IntoResponse, Request, Response,
};
use rocket::tokio;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    // from_filename("dev.env").expect("Failed to load env file");
    dotenv().ok();
    run(service_fn(handler)).await
}

async fn handler(event: Request) -> Result<impl IntoResponse, Error> {
    let path = event.uri().path();
    let method = event.method().as_str();
    match (method, path) {
        ("POST", "/fiat-transaction") => fiat_transaction(event).await,
        ("POST", "/crypto-transaction") => crypto_transaction(event).await,
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::Text("Not found".to_string()))
            .unwrap()),
    }
}
