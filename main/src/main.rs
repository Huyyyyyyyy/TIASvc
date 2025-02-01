use dotenv::dotenv;
use framework::handlers::transfers;
use rocket::tokio;

#[macro_use]
extern crate rocket;

#[tokio::main]
pub async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let _rocket = rocket::build()
        .mount("/", routes![transfers])
        .launch()
        .await?;
    Ok(())
}
