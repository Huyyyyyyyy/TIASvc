use ethers::prelude::*;
use std::env;

pub struct InfuraRepository {
    pub provider: Provider<Http>,
    pub base_url: String,
    pub api_key: String,
}

impl InfuraRepository {
    pub fn new() -> Self {
        let base_url = env::var("INFURA_BASE_URL").expect("Infura base url must be set");
        Self {
            provider: Provider::<Http>::try_from(&base_url).unwrap(),
            base_url,
            api_key: env::var("INFURA_API_KEY").expect("Infura api key must be set"),
        }
    }
}
