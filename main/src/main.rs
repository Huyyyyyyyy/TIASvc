use dotenv::from_filename;
use framework::handlers::transfers;
use rocket::tokio;

#[macro_use]
extern crate rocket;

#[tokio::main]
pub async fn main() -> Result<(), rocket::Error> {
    from_filename("dev.env").expect("Failed to load env file");
    let _rocket = rocket::build()
        .mount("/", routes![transfers])
        .launch()
        .await?;
    Ok(())
}
