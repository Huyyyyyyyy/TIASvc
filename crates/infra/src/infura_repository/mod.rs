use anyhow::{anyhow, Result};
use async_trait::async_trait;
use domain::repository::web3_repository::Web3Repository;
use ethers::{abi::Abi, prelude::*};
use std::{env, sync::Arc};

use crate::contract_abi::{CT_LINK, CT_USDC};

pub struct InfuraRepository {
    pub provider: Provider<Http>,
    pub base_url: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ContractABI {
    USDC,
    LINK,
    NONE,
}

impl ContractABI {
    pub fn get_contract_abi(&self) -> String {
        match self {
            ContractABI::USDC => CT_USDC.to_string(),
            ContractABI::LINK => CT_LINK.to_string(),
            ContractABI::NONE => "".to_string(),
        }
    }

    pub fn get_contract_address(&self) -> String {
        match self {
            ContractABI::USDC => {
                env::var("CONTRACT_USDC").expect("USDC contract address must be set")
            }
            ContractABI::LINK => {
                env::var("CONTRACT_LINK").expect("LINK contract address must be set")
            }
            _ => "".to_string(),
        }
    }

    pub fn map_token_contract(chain: &str) -> ContractABI {
        match chain.to_lowercase().as_str() {
            "usdc" => ContractABI::USDC,
            "link" => ContractABI::LINK,
            _ => ContractABI::NONE,
        }
    }
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

    async fn establish_signer_wallet(
        &self,
        signer_private_key: &str,
        contract: ContractABI,
    ) -> Result<Contract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        //build the infura provider
        let rpc_url = format!("{}/v3/{}", self.base_url, self.api_key);
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let chain_id = provider.clone().get_chainid().await.unwrap();

        //build the wallet base on signer private key
        let wallet: LocalWallet = signer_private_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id.as_u64());

        //combine both of those above to a client provider
        let client = Arc::new(SignerMiddleware::new(
            provider.clone(),
            wallet.with_chain_id(chain_id.as_u64()),
        ));

        //get abi contract
        let abi_string = contract.get_contract_abi();
        let abi: Abi = serde_json::from_str(&abi_string).unwrap();

        //get contract address base on chain -> this address will be taken on the .env file
        let contract_address = contract.get_contract_address().parse::<Address>().unwrap();

        //build the contract token
        let contract = Contract::new(contract_address, abi, client);
        Ok(contract)
    }
}

#[async_trait]
impl Web3Repository for InfuraRepository {
    async fn transfer_erc20_token(
        &self,
        sender_private_key: &str,
        recipient_address: &str,
        amount: &str,
        chain: &str,
    ) -> Result<String> {
        let contract = ContractABI::map_token_contract(chain);
        let contract = self
            .establish_signer_wallet(sender_private_key, contract)
            .await?;

        let whole_amount = amount
            .parse::<f64>()
            .map_err(|e| anyhow!("Failed to parse amount: {}", e))?;

        let decimals: u8 = contract.method::<(), u8>("decimals", ())?.call().await?;

        let multiplier = 10f64.powi(decimals as i32);
        let decimal_amount = U256::from((whole_amount * multiplier) as u128);

        let recipient = recipient_address
            .parse::<Address>()
            .map_err(|e| anyhow!("Invalid recipient address: {}", e))?;

        let transaction =
            contract.method::<(Address, U256), H256>("transfer", (recipient, decimal_amount))?;

        let pending_transaction = transaction.send().await?;
        let receipt = pending_transaction.await?;

        let json_str = serde_json::to_string(&receipt)
            .map_err(|e| anyhow!("Failed to serialize transaction receipt: {}", e))?;

        Ok(json_str)
    }
}
