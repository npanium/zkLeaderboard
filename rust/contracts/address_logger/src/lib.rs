#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::Address;
use stylus_sdk::{
    alloy_primitives::U256, alloy_sol_types::sol, block, evm, msg, prelude::*, ArbResult,
};

sol! {
    event AddressLog(
        address indexed operator,
        address[] addresses,
        uint256 timestamp,
        uint256 batch_size
    );
}

#[storage]
#[entrypoint]
pub struct AddressLogger;

#[public]
impl AddressLogger {
    pub fn log_addresses(&mut self, addresses: Vec<Address>) -> ArbResult {
        let timestamp = block::timestamp();
        let batch_size = addresses.len() as u64;

        evm::log(AddressLog {
            operator: msg::sender(),
            addresses,
            timestamp: U256::from(timestamp),
            batch_size: U256::from(batch_size),
        });

        Ok(Vec::new())
    }
}
