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
