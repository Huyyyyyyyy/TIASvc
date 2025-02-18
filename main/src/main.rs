use dotenv::dotenv;
use framework::handler::lambda_handler;
use lambda_http::{aws_lambda_events::encodings::Error, run, service_fn};
use rocket::tokio;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    //load .env file
    dotenv().ok();
    run(service_fn(lambda_handler)).await
}
