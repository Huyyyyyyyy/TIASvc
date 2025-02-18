use crate::contract_abi::{CT_LINK, CT_ROUTER02, CT_USDC, CT_WETH};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use domain::repository::web3_repository::Web3Repository;
use ethers::{
    abi::Abi,
    core::rand::thread_rng,
    prelude::*,
    utils::{self, parse_units},
};
use std::{
    env,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
pub struct InfuraRepository {
    pub provider: Provider<Http>,
    pub base_url: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContractABI {
    USDC,
    LINK,
    ETH,
    WETH,
    ROUTER02,
    NONE,
}

impl ContractABI {
    pub fn get_contract_abi(&self) -> String {
        match self {
            ContractABI::USDC => CT_USDC.to_string(),
            ContractABI::LINK => CT_LINK.to_string(),
            ContractABI::ETH => "".to_string(),
            ContractABI::WETH => CT_WETH.to_string(),
            ContractABI::ROUTER02 => CT_ROUTER02.to_string(),
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
            ContractABI::WETH => {
                env::var("CONTRACT_WETH").expect("WETH contract address must be set")
            }
            ContractABI::ROUTER02 => {
                env::var("CONTRACT_ROUTER02").expect("ROUTER02 contract address must be set")
            }
            _ => "".to_string(),
        }
    }

    pub fn map_token_contract(chain: &str) -> ContractABI {
        match chain.to_lowercase().as_str() {
            "usdc" => ContractABI::USDC,
            "link" => ContractABI::LINK,
            "eth" => ContractABI::ETH,
            "weth" => ContractABI::WETH,
            "router02" => ContractABI::ROUTER02,
            _ => ContractABI::NONE,
        }
    }
}

//swap method
#[derive(Debug, Clone, Copy)]
pub enum SwapMethod {
    SwapTokensForExactETH,
    SwapETHForExactTokens,
    SwapTokensForExactTokens,
}

impl SwapMethod {
    fn map_swap_method(from: ContractABI, to: ContractABI) -> Result<SwapMethod> {
        match (from, to) {
            (ContractABI::WETH, t) if t != ContractABI::WETH => {
                Ok(SwapMethod::SwapETHForExactTokens)
            }
            (f, ContractABI::WETH) if f != ContractABI::WETH => {
                Ok(SwapMethod::SwapTokensForExactETH)
            }
            (f, t) if f != ContractABI::WETH && t != ContractABI::WETH => {
                Ok(SwapMethod::SwapTokensForExactTokens)
            }
            //all the others not match -> say we don't support
            _ => Err(anyhow!("Not support pairs to swap")),
        }
    }

    fn to_string(&self) -> String {
        match self {
            SwapMethod::SwapETHForExactTokens => "swapETHForExactTokens".to_string(),
            SwapMethod::SwapTokensForExactETH => "swapTokensForExactETH".to_string(),
            SwapMethod::SwapTokensForExactTokens => "swapTokensForExactTokens".to_string(),
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

    async fn establish_contract_router(
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

    fn get_valid_timestamp(&self, future_millis: u128) -> u128 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_millis = since_epoch.as_millis().checked_add(future_millis).unwrap();
        time_millis
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

    async fn get_wallet(&self, signer_private_key: &str) -> Result<String> {
        //do it like establishing a client wallet
        let chain_id = self.provider.clone().get_chainid().await?;
        let wallet = signer_private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        //just return the wallet address as string
        let address = format!("{:?}", wallet.address());
        Ok(address)
    }

    async fn create_wallet(&self) -> Result<(String, String)> {
        let wallet = LocalWallet::new(&mut thread_rng());
        let private_key = hex::encode(wallet.signer().to_bytes());
        let address = format!("{:?}", wallet.address());

        Ok((address, private_key))
    }

    async fn swap(
        &self,
        from_token: &str,
        to_token: &str,
        amount: &str,
        signer_private_key: &str,
    ) -> Result<String> {
        //establish the client provider
        let client = self.establish_signer_wallet(signer_private_key).await?;
        let signer_address = client.address();

        //establish contract of uniswap
        let contract_router = self
            .establish_contract_router(client.clone(), ContractABI::ROUTER02)
            .await?;
        let router20_address = contract_router.address();

        //get from contract
        let mut from_detect = ContractABI::map_token_contract(&from_token);
        //need to map again because ETH use WETH to swap
        if from_detect == ContractABI::ETH {
            from_detect = ContractABI::WETH;
        }
        let from_contract = self
            .establish_contract_erc20(client.clone(), from_detect)
            .await?;
        let from_address = from_contract.address();

        //get destination contract
        let mut destination_detect = ContractABI::map_token_contract(&to_token);
        //need to map again because ETH use WETH to swap
        if destination_detect == ContractABI::ETH {
            destination_detect = ContractABI::WETH;
        }
        let destination_contract = self
            .establish_contract_erc20(client.clone(), destination_detect)
            .await?;
        let destination_address = destination_contract.address();

        //process the amount
        let decimals: u8 = from_contract.method("decimals", ())?.call().await?;
        let approval_amount = parse_units(amount, decimals as i32)?.to_string();
        let range_expected: Vec<U256> = contract_router
            .method(
                "getAmountsOut",
                (
                    U256::from_dec_str(&approval_amount).unwrap(),
                    vec![from_address, destination_address],
                ),
            )?
            .call()
            .await?;
        let expected_amount = range_expected
            .last()
            .ok_or_else(|| anyhow!("No expected amount"))?;
        let amount_out_min = expected_amount * U256::from(95) / U256::from(100);

        //send request to usdc contract for approve amount
        let approve_tx = from_contract.method::<_, H256>(
            "approve",
            (
                router20_address,
                U256::from_dec_str(&approval_amount).unwrap(),
            ),
        )?;
        let pending_approve_tx = approve_tx.send().await?;
        pending_approve_tx.await?;

        //timestamp deadline
        let valid_time = self.get_valid_timestamp(300000);
        let u256_timestamp = U256::from_dec_str(&valid_time.to_string()).unwrap();

        //send request to swap token
        let method = SwapMethod::map_swap_method(from_detect, destination_detect)?;
        let swap_tx = match method {
            SwapMethod::SwapETHForExactTokens => contract_router.method::<_, H256>(
                &SwapMethod::to_string(&method),
                (
                    U256::from_dec_str(&approval_amount).unwrap(),
                    vec![from_address, destination_address],
                    signer_address,
                    u256_timestamp,
                ),
            )?,
            _ => contract_router.method::<_, H256>(
                &SwapMethod::to_string(&method),
                (
                    U256::from_dec_str(&approval_amount).unwrap(),
                    amount_out_min,
                    vec![from_address, destination_address],
                    signer_address,
                    u256_timestamp,
                ),
            )?,
        };
        let pending_swap_tx = swap_tx.send().await?;
        let minted_swap_tx = pending_swap_tx.await?;
        if minted_swap_tx.is_some() {
            let rs = format!("{:?}", minted_swap_tx.unwrap().transaction_hash);
            Ok(rs)
        } else {
            Ok("".to_string())
        }
    }
}
