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
        function logAddresses(address[] memory addresses) external returns (uint8[] memory)
        function placeBet(address selected_address, bool position) external payable returns (uint8[] memory)
        function getBet(uint256 index) external view returns (address, address, bool, uint256)
        function getBetCount() external view returns (uint256)
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

    pub async fn log_addresses(&self, addresses: Vec<Address>) -> Result<Vec<u8>> {
        let tx = self
            .contract
            .log_addresses(addresses)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(tx
            .logs
            .into_iter()
            .next()
            .map(|log| log.data.to_vec())
            .unwrap_or_default())
    }

    pub async fn place_bet(
        &self,
        selected_address: Address,
        position: bool,
        value: U256,
    ) -> Result<Vec<u8>> {
        let tx = self
            .contract
            .place_bet(selected_address, position)
            .value(value)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(tx
            .logs
            .into_iter()
            .next()
            .map(|log| log.data.to_vec())
            .unwrap_or_default())
    }

    pub async fn get_bet(&self, index: U256) -> Result<(Address, Address, bool, U256)> {
        Ok(self.contract.get_bet(index).call().await?)
    }

    pub async fn get_bet_count(&self) -> Result<U256> {
        Ok(self.contract.get_bet_count().call().await?)
    }
}
