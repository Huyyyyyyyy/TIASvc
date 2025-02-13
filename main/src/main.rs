use dotenv::from_filename;
use framework::handlers::{crypto_transaction, fiat_transaction};
use rocket::tokio;

#[macro_use]
extern crate rocket;

#[tokio::main]
pub async fn main() -> Result<(), rocket::Error> {
    from_filename("dev.env").expect("Failed to load env file");
    let _rocket = rocket::build()
        .mount("/", routes![fiat_transaction, crypto_transaction])
        .launch()
        .await?;
    Ok(())
}
