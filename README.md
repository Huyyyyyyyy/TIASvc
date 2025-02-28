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
