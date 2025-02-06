#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::Address;
use stylus_sdk::{
    alloy_primitives::U256,
    alloy_sol_types::sol,
    block, evm, msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageU256, StorageVec},
    ArbResult,
};

sol! {
    event AddressLog(
        address indexed operator,
        address[] addresses,
        uint256 timestamp,
        uint256 batch_size
    );

    event BetPlaced(
        address indexed bettor,
        address indexed selectedAddress,
        bool position,
        uint256 amount
    );
}

#[storage]
#[entrypoint]
pub struct AddressLogger {
    addresses: StorageVec<StorageAddress>,
    bets: StorageVec<Bet>,
    // window_active: StorageBool,
    // bet_duration: StorageU256,
}

#[storage]
pub struct Bet {
    bettor: StorageAddress,
    selected_address: StorageAddress,
    position: StorageBool,
    amount: StorageU256,
}

#[public]
impl AddressLogger {
    pub fn log_addresses(&mut self, addresses: Vec<Address>) -> ArbResult {
        let timestamp = block::timestamp();
        let batch_size = addresses.len() as u64;

        // Store addresses
        for addr in &addresses {
            self.addresses.push(*addr);
        }

        evm::log(AddressLog {
            operator: msg::sender(),
            addresses,
            timestamp: U256::from(timestamp),
            batch_size: U256::from(batch_size),
        });

        Ok(Vec::new())
    }

    #[payable]
    pub fn place_bet(&mut self, selected_address: Address, position: bool) -> ArbResult {
        let amount = msg::value();
        let bettor = msg::sender();

        let mut new_bet = self.bets.grow();
        new_bet.bettor.set(bettor);
        new_bet.selected_address.set(selected_address);
        new_bet.position.set(position);
        new_bet.amount.set(amount);

        evm::log(BetPlaced {
            bettor,
            selectedAddress: selected_address,
            position,
            amount,
        });

        Ok(Vec::new())
    }

    // Helper function for testing
    pub fn get_bet(&self, index: U256) -> Result<(Address, Address, bool, U256), Vec<u8>> {
        let idx = index.as_limbs()[0] as usize;
        if idx >= self.bets.len() {
            return Err(Vec::from(b"Index out of bounds"));
        }
        let bet = self.bets.getter(idx).unwrap();
        Ok((
            bet.bettor.get(),
            bet.selected_address.get(),
            bet.position.get(),
            bet.amount.get(),
        ))
    }

    pub fn get_bet_count(&self) -> U256 {
        U256::from(self.bets.len())
    }
}
