use crate::contract_abi::{CT_LINK, CT_USDC};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use domain::repository::web3_repository::Web3Repository;
use ethers::{abi::Abi, prelude::*, utils};
use std::{env, sync::Arc};
pub struct InfuraRepository {
    pub provider: Provider<Http>,
    pub base_url: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ContractABI {
    USDC,
    LINK,
    ETH,
    NONE,
}

impl ContractABI {
    pub fn get_contract_abi(&self) -> String {
        match self {
            ContractABI::USDC => CT_USDC.to_string(),
            ContractABI::LINK => CT_LINK.to_string(),
            ContractABI::ETH => "".to_string(),
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
            "eth" => ContractABI::ETH,
            _ => ContractABI::NONE,
        }
    }
}

impl InfuraRepository {
    pub fn new() -> Self {
        //build the infura provider for reusable
        let base_url = env::var("INFURA_BASE_URL").expect("Infura base url must be set");
        let api_key = env::var("INFURA_API_KEY").expect("Infura api key must be set");
        let rpc_url = format!("{}/v3/{}", base_url, api_key);
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        Self {
            provider,
            base_url,
            api_key,
        }
    }

    async fn parse_amount(
        &self,
        contract: Contract<SignerMiddleware<Provider<Http>, LocalWallet>>,
        amount: &str,
    ) -> Result<U256> {
        let whole_amount = amount
            .parse::<f64>()
            .map_err(|e| anyhow!("Failed to parse amount: {}", e))?;

        let decimals: u8 = contract.method::<(), u8>("decimals", ())?.call().await?;

        let multiplier = 10f64.powi(decimals as i32);
        let decimal_amount = U256::from((whole_amount * multiplier) as u128);
        Ok(decimal_amount)
    }

    async fn establish_contract_erc20(
        &self,
        client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
        contract: ContractABI,
    ) -> Result<Contract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        //get abi contract
        let abi_string = contract.get_contract_abi();
        let abi: Abi = serde_json::from_str(&abi_string).unwrap();

        //get contract address base on chain -> this address will be taken on the .env file
        let contract_address = contract.get_contract_address().parse::<Address>().unwrap();

        //build the contract token
        let contract = Contract::new(contract_address, abi, client);
        Ok(contract)
    }

    async fn establish_signer_wallet(
        &self,
        signer_private_key: &str,
    ) -> Result<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        //build the infura provider
        let chain_id = self.provider.clone().get_chainid().await.unwrap();

        //build the wallet base on signer private key
        let wallet: LocalWallet = signer_private_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id.as_u64());

        //combine both of those above to a client provider
        let client = Arc::new(SignerMiddleware::new(
            self.provider.clone(),
            wallet.with_chain_id(chain_id.as_u64()),
        ));

        Ok(client)
    }
}

#[async_trait]
impl Web3Repository for InfuraRepository {
    async fn transfer_token(
        &self,
        sender_private_key: &str,
        recipient_address: &str,
        amount: &str,
        chain: &str,
    ) -> Result<String> {
        let contract_abi = ContractABI::map_token_contract(chain);
        let client = self.establish_signer_wallet(sender_private_key).await?;
        let recipient = recipient_address
            .parse::<Address>()
            .map_err(|e| anyhow!("Invalid recipient address: {}", e))?;

        //check to know the contract is ETH or else
        let rs = match contract_abi {
            //if it is ETH so no need to create contract erc20
            ContractABI::ETH => {
                let tx = TransactionRequest::new()
                    .to(recipient)
                    .value(U256::from(utils::parse_ether(amount)?));

                let pending_tx = client.send_transaction(tx, None).await?;
                let receipt = pending_tx.await?.unwrap_or_default();

                format!("{:?}", receipt.transaction_hash)
            }
            //if it is not ETH so must create erc20 contract
            _ => {
                let contract = self.establish_contract_erc20(client, contract_abi).await?;
                let decimal_amount = self.parse_amount(contract.clone(), amount).await?;

                let tx = contract
                    .method::<(Address, U256), H256>("transfer", (recipient, decimal_amount))?;

                let pending_tx = tx.send().await?;
                let receipt = pending_tx.await?.unwrap_or_default();

                format!("{:?}", receipt.transaction_hash)
            }
        };
        Ok(rs)
    }

    async fn get_balance(&self, signer_private_key: &str, chain: &str) -> Result<String> {
        let client = self.establish_signer_wallet(signer_private_key).await?;
        let contract = ContractABI::map_token_contract(chain);
        let address = client.address();

        match contract {
            ContractABI::ETH => {
                let balance = self.provider.clone().get_balance(address, None).await?;
                let formatted_balance = utils::format_units(balance, 18)?;
                Ok(formatted_balance.to_string())
            }
            _ => {
                let token_contract = self.establish_contract_erc20(client, contract).await?;
                // Retrieve decimals of the contract token for convert balance
                let decimals: u8 = token_contract.method("decimals", ())?.call().await?;
                //retrieve and convert token balance by decimals
                let balance: U256 = token_contract.method("balanceOf", address)?.call().await?;
                let formatted_balance = utils::format_units(balance, decimals as i32)?;
                Ok(formatted_balance.to_string())
            }
        }
    }
}
