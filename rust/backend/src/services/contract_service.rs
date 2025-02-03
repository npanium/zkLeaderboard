use anyhow::Result;
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, H256, U256},
};
use std::sync::Arc;

// Generate contract bindings
abigen!(
    HashStorage,
    r#"[
        function storeHashRecord(bytes32 hash, uint256 timestamp, uint256 record_count) external returns (uint8[] memory)
    ]"#
);
#[derive(Clone)]
pub struct ContractService {
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract: HashStorage<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl ContractService {
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

        let contract = HashStorage::new(address, client.clone());

        Ok(Self { client, contract })
    }

    pub async fn store_hash(
        &self,
        hash: [u8; 32],
        timestamp: i64,
        record_count: usize,
    ) -> Result<Vec<u8>> {
        let hash_bytes = H256::from(hash);
        let timestamp = U256::from(timestamp as u64);
        let record_count = U256::from(record_count);

        let tx = self
            .contract
            .store_hash_record(hash_bytes.into(), timestamp, record_count)
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
}
