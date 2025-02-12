use anyhow::Result;
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
};
use std::sync::Arc;

// Generate contract bindings
abigen!(
    AddrLogger,
    r#"[

    function init(address operator, address treasury, address token) external returns (uint8[] memory)
    function startBettingWindow(address[] memory addresses) external returns (uint8[] memory)
    function closeBettingWindow() external returns (uint8[] memory)
    function placeBetWithSignature(address bettor, address selected_address, bool position, uint256 amount) external returns (uint8[] memory)
    function placeBet(address bettor, address selected_address, bool position, uint256 amount) external returns (uint8[] memory)
    function getWindowActive() external view returns (bool)
    function getBet(uint256 index) external view returns (address, address, bool, uint256)
    function getBetCount() external view returns (uint256)
    function processPayouts(bool[] memory winners) external
    function isValidAddress(address _address) external view returns (bool)
    function getOperator() external view returns (address)
    function getTreasury() external view returns (address)
    function getToken() external view returns (address)
    function getUpAmount(uint256 addr_index) external view returns (uint256)
    function getDownAmount(uint256 addr_index) external view returns (uint256)

    ]"#
);

#[derive(Clone)]

pub struct AddrLoggerContractService {
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract: AddrLogger<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl AddrLoggerContractService {
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

        let contract = AddrLogger::new(address, client.clone());

        Ok(Self { client, contract })
    }

    pub async fn init(
        &self,
        operator: Address,
        treasury: Address,
        token: Address,
    ) -> Result<String> {
        let tx = self
            .contract
            .init(operator, treasury, token)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(format!("{:#x}", tx.transaction_hash))
        // Ok(tx
        //     .logs
        //     .into_iter()
        //     .next()
        //     .map(|log| log.data.to_vec())
        //     .unwrap_or_default())
    }

    pub async fn start_betting_window(&self, addresses: Vec<Address>) -> Result<String> {
        let tx = self
            .contract
            .start_betting_window(addresses)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(format!("{:#x}", tx.transaction_hash))
    }

    pub async fn close_betting_window(&self) -> Result<String> {
        let tx = self
            .contract
            .close_betting_window()
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(format!("{:#x}", tx.transaction_hash))
    }

    pub async fn place_bet(
        &self,
        bettor: Address,
        selected_address: Address,
        position: bool,
        amount: U256,
    ) -> Result<String> {
        let tx = self
            .contract
            .place_bet(bettor, selected_address, position, amount)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(format!("{:#x}", tx.transaction_hash))
    }

    pub async fn get_window_active(&self) -> Result<bool> {
        Ok(self.contract.get_window_active().call().await?)
    }

    pub async fn get_bet(&self, index: U256) -> Result<(Address, Address, bool, U256)> {
        Ok(self.contract.get_bet(index).call().await?)
    }

    pub async fn get_bet_count(&self) -> Result<U256> {
        Ok(self.contract.get_bet_count().call().await?)
    }

    pub async fn process_payouts(&self, winners: Vec<bool>) -> Result<String> {
        let tx = self
            .contract
            .process_payouts(winners)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(format!("{:#x}", tx.transaction_hash))
    }

    pub async fn is_valid_address(&self, address: Address) -> Result<bool> {
        Ok(self.contract.is_valid_address(address).call().await?)
    }

    pub async fn get_operator(&self) -> Result<Address> {
        Ok(self.contract.get_operator().call().await?)
    }

    pub async fn get_treasury(&self) -> Result<Address> {
        Ok(self.contract.get_treasury().call().await?)
    }

    pub async fn get_token(&self) -> Result<Address> {
        Ok(self.contract.get_token().call().await?)
    }

    pub async fn get_up_amount(&self, addr_index: U256) -> Result<U256> {
        Ok(self.contract.get_up_amount(addr_index).call().await?)
    }

    pub async fn get_down_amount(&self, addr_index: U256) -> Result<U256> {
        Ok(self.contract.get_down_amount(addr_index).call().await?)
    }
}
