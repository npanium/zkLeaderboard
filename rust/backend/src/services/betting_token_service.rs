use anyhow::Result;
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
};
use log::debug;
use std::sync::Arc;

// Generate contract bindings
abigen!(
    BettingToken,
    r#"[
        function mint(uint256 value) external
        function mintTo(address to, uint256 value) external
        function burn(uint256 value) external
        function balanceOf(address account) external view returns (uint256)
        function transfer(address to, uint256 amount) external returns (bool)
        function transferFrom(address from, address to, uint256 amount) external returns (bool)
        function approve(address spender, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
    ]"#
);

#[derive(Clone)]
pub struct BettingTokenService {
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract: BettingToken<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl BettingTokenService {
    pub async fn new(rpc_url: &str, private_key: &str, contract_address: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet = private_key.parse::<LocalWallet>()?;
        let chain_id = provider.get_chainid().await?.as_u64();
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(chain_id),
        ));

        let address = contract_address
            .parse::<Address>()
            .map_err(|e| anyhow::anyhow!("Failed to parse contract address: {}", e))?;

        let contract = BettingToken::new(address, client.clone());

        Ok(Self { client, contract })
    }

    const DECIMALS: u32 = 18;

    // Helper function to convert from user-friendly amount to token amount with decimals
    fn to_token_amount(amount: u64) -> U256 {
        U256::from(amount) * U256::from(10).pow(U256::from(Self::DECIMALS))
    }

    // Helper function to convert from token amount with decimals to user-friendly amount
    fn from_token_amount(amount: U256) -> u64 {
        (amount / U256::from(10).pow(U256::from(Self::DECIMALS))).as_u64()
    }

    pub async fn mint(&self, value: u64) -> Result<Vec<u8>> {
        let amount = Self::to_token_amount(value);
        print!("[betting token service] Amount: {}", amount);
        let tx = self
            .contract
            .mint(amount)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(tx.transaction_hash.as_bytes().to_vec())
    }

    pub async fn mint_to(&self, to: Address, value: u64) -> Result<Vec<u8>> {
        let amount = Self::to_token_amount(value);
        debug!("Raw amount with decimals: {}", amount);

        let tx = self
            .contract
            .mint_to(to, amount)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        println!("Transaction: {:?}", tx);
        Ok(tx.transaction_hash.as_bytes().to_vec())
    }

    pub async fn burn(&self, value: u64) -> Result<Vec<u8>> {
        let amount = Self::to_token_amount(value);
        let tx = self
            .contract
            .burn(amount)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(tx.transaction_hash.as_bytes().to_vec())
    }

    pub async fn balance_of(&self, account: Address) -> Result<u64> {
        let balance = self.contract.balance_of(account).call().await?;
        Ok(Self::from_token_amount(balance))
    }
}
