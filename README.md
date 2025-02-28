TIASvc

TIASvc is a Rust-based service designed as a modular and scalable solution for interacting with Celestia blockchain networks and APIs. 
Built as a Cargo workspace, TIASvc separates concerns into multiple crates—each handling distinct layers of the application. 
It integrates with external services such as Circle Mint, Celestia RPC, Infura, and various smart contract networks.
Making it suitable as a backend service for token-related operations and blockchain interactions.

Features
- Modular Architecture:
Organized as a Cargo workspace with dedicated crates for application logic, domain modeling, framework integration, and infrastructure implementations.

- Blockchain Integrations:
Provides configuration and support for connecting to blockchain networks via Celestia RPC and Infura.
```rs
pub struct CelestiaRepository {
    pub client: Client,
    pub url: String,
    pub auth_token: String,
}

impl CelestiaRepository {
    pub async fn new() -> Self {
        let url = std::env::var("CELESTIA_RPC_URL").expect("Celestia rpc url must be set");
        let auth_token =
            std::env::var("CELESTIA_AUTH_TOKEN").expect("Celestia auth token must be set");
        let client = Client::new(&url, Some(&auth_token)).await.unwrap();
        client.header_wait_for_height(2).await.unwrap();
        Self {
            client,
            url,
            auth_token,
        }
    }

    pub async fn write_lock(&self) -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().await
    }

    pub fn address_to_namespace(&self, address: &str) -> Result<Namespace> {
        // Remove "0x" if present.
        let addr = address.strip_prefix("0x").unwrap_or(address);
        // Decode the hex string into raw bytes.
        let decoded = BASE64_STANDARD.decode(addr)?;
        if decoded.len() < 8 {
            return Err(anyhow!("Address decoded bytes less than 8"));
        }
        // Take the first 8 bytes.
        let mut ns = [0u8; 8];
        ns.copy_from_slice(&decoded[..8]);
        let namespace: Namespace = Namespace::new_v0(&ns)?;
        Ok(namespace)
    }
}

#[async_trait]
impl ChainRepository<Blob, Namespace> for CelestiaRepository {
    async fn submit(&self, blobs: &[Blob]) -> Result<u64> {
        let _guard = self.write_lock().await;
        let height = self.client.blob_submit(blobs, TxConfig::default()).await?;
        Ok(height)
    }

    async fn get_all(
        &self,
        namespace: &[Namespace],
        height: u64,
    ) -> Result<Vec<TransactionHistoryResponseDTO>> {
        let blobs = self.client.blob_get_all(height, namespace).await?;
        let mut rs = Vec::<TransactionHistoryResponseDTO>::new();
        match blobs {
            Some(blobs) => {
                for blob in blobs {
                    println!("found at height {:?}", height);
                    let data = self.revert_blob(&blob)?;
                    rs.push(data);
                }
            }
            None => {}
        }
        Ok(rs)
    }

    fn revert_blob(&self, blob: &Blob) -> Result<TransactionHistoryResponseDTO> {
        // Convert the stored bytes into a UTF-8 string; this string is the base64 encoded JSON.
        let encoded_str = String::from_utf8(blob.data.clone())?;

        // Decode the base64 string back to the original JSON string bytes.
        let decoded_bytes = BASE64_STANDARD.decode(encoded_str.as_bytes())?;

        // Convert the decoded bytes into a UTF-8 string (the JSON string).
        let json_str = String::from_utf8(decoded_bytes)?;

        // Deserialize the JSON string back into a serde_json::Value.
        let data = serde_json::from_str::<TransactionHistoryResponseDTO>(&json_str).unwrap();

        Ok(data)
    }

    async fn build_blob(&self, namespace: &str, data: Value) -> Result<Blob> {
        let namespace: Namespace = self.address_to_namespace(namespace)?;
        let data_str = serde_json::to_string(&data)?;
        let encoded_data: Vec<u8> = BASE64_STANDARD.encode(data_str).as_bytes().to_vec();
        let blob: Blob = Blob::new(namespace, encoded_data, AppVersion::V2)?;
        Ok(blob)
    }
}
```

- Smart Contract Interactions:
Prepares to interact with token contracts (e.g., USDC, LINK, WETH) and Uniswap's router contract.

- API Integrations:
Configurable integration with the Circle Mint API for token minting and related operations.

- Database Connectivity:
Uses a PostgreSQL database for persistence, with connection details provided via environment variables.


Getting Started
Prerequisites
Rust and Cargo:
Ensure you have the latest stable version of Rust installed.

PostgreSQL:
Install and run PostgreSQL for database operations.

External API Credentials:
Obtain API keys and endpoint URLs for Circle Mint, Celestia, and Infura as needed.

Installation
Clone the Repository:
```sh
git clone https://github.com/Huyyyyyyyy/TIASvc.git
cd TIASvc
```
Set Up Environment Variables:

Copy the provided .env.example file to a new .env file and fill in the required values:
Please fill out all the variables of the env
```sh
cp .env.example .env
```


Configuration
The project uses environment variables to configure external services. Below is an overview of the variables defined in .env.example:

```
Circle API:
CIRCLE_MINT_API_KEY: Your Circle Mint API key.
CIRCLE_MINT_BASE_URL: Base URL for the Circle API.
Celestia RPC:

CELESTIA_RPC_URL: URL for the Celestia RPC endpoint.
CELESTIA_AUTH_TOKEN: Authorization token for Celestia.
Infura:

INFURA_API_KEY: Your Infura API key.
INFURA_BASE_URL: Base URL for Infura.
Smart Contract Addresses:

CONTRACT_USDC: Address for the USDC token contract.
CONTRACT_LINK: Address for the LINK token contract.
CONTRACT_WETH: Address for the WETH token contract.
CONTRACT_ROUTER02: Address for the Uniswap Router contract.
Database:

DATABASE_URL: PostgreSQL connection string.
```

Project Structure
The repository is structured as a Cargo workspace with the following members:
```
crates/app:
Contains the core application logic and service orchestration.

crates/domain:
Defines domain models and business logic.

crates/framework:
Integrates with external frameworks and libraries, currently we use Rocket as framework providing the API template .
And this API will be processed on Serverless Lambda function.
For now we haven't had a debug environment yet .
Instead we have an available Lambda Function for you guys of function UI testing purposes.

crates/infra:
Houses infrastructure-related implementations (e.g., API clients, repositories such as Celestia RPC , Circle Mint, Infura API).

main:
Contains the entry point for running the service.
```
