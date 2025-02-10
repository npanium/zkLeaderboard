#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use alloy_sol_types::{
    sol,
    sol_data::{Address as SOLAddress, FixedBytes as SolFixedBytes, *},
    SolType,
};

use stylus_sdk::{
    abi::Bytes,
    alloy_primitives::{address, Address, FixedBytes, U256},
    block,
    call::{self, Call},
    contract::address,
    crypto::keccak,
    evm, msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageMap, StorageU256, StorageVec},
    ArbResult,
};
type ECRECOVERType = (
    SolFixedBytes<32>,
    Uint<8>,
    SolFixedBytes<32>,
    SolFixedBytes<32>,
);

const FEE_PERCENTAGE: u64 = 10;
const ECRECOVER: Address = address!("0000000000000000000000000000000000000001");
const SIGNED_MESSAGE_HEAD: &'static str = "\x19Ethereum Signed Message:\n32";

sol_interface! {
    interface IERC20  {
        function balanceOf(address owner) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        function approve(address spender, uint256 value) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
    }

}

sol! {
    event WindowStarted(
        address indexed operator,
        address[] validAddresses,
        uint256 timestamp
    );

    event WindowClosed(
        address indexed operator,
        uint256 timestamp
    );

    event BetPlaced(
        address indexed bettor,
        address indexed selectedAddress,
        bool position,
        uint256 amount
    );

    event PayoutProcessed(
        address indexed bettor,
        uint256 amount,
        bool isWinner
    );
    error EcrecoverCallError();
    error InvalidSignatureLength();
}

#[storage]
#[entrypoint]
pub struct AddressLogger {
    window_active: StorageBool,
    valid_addresses: StorageVec<StorageAddress>,
    bets: StorageVec<Bet>,
    operator: StorageAddress,                    // Added to restrict control
    treasury: StorageAddress,                    // Address to collect fees
    address_up_amounts: StorageVec<StorageU256>, // Total UP amounts per address (after fees)
    address_down_amounts: StorageVec<StorageU256>, // Total DOWN amounts per address (after fees)
    token_address: StorageAddress,
    nonces: StorageMap<Address, StorageU256>,
}

#[derive(SolidityError)]
pub enum AddressLoggerError {
    EcrecoverCallError(EcrecoverCallError),
    InvalidSignatureLength(InvalidSignatureLength),
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
    pub fn init(&mut self, operator: Address, treasury: Address, token: Address) -> ArbResult {
        // Check if already initialized
        if self.operator.get() != Address::ZERO {
            return Err(Vec::from(b"Already initialized"));
        }

        // Set initial values
        self.operator.set(operator);
        self.treasury.set(treasury);
        self.token_address.set(token);
        self.window_active.set(false);

        Ok(Vec::new())
    }

    pub fn start_betting_window(&mut self, addresses: Vec<Address>) -> ArbResult {
        // Only operator can start window
        if msg::sender() != self.operator.get() {
            return Err(Vec::from(b"Not authorized"));
        }

        // Cannot start new window if one is active
        if self.window_active.get() {
            return Err(Vec::from(b"Window already active"));
        }

        // Clear previous addresses and amounts, store new ones
        while self.valid_addresses.pop().is_some() {}
        while self.address_up_amounts.pop().is_some() {}
        while self.address_down_amounts.pop().is_some() {}

        for addr in &addresses {
            self.valid_addresses.push(*addr);
            self.address_up_amounts.push(U256::ZERO);
            self.address_down_amounts.push(U256::ZERO);
        }

        // Activate window
        self.window_active.set(true);

        // Emit event
        evm::log(WindowStarted {
            operator: msg::sender(),
            validAddresses: addresses,
            timestamp: U256::from(block::timestamp()),
        });

        Ok(Vec::new())
    }

    pub fn close_betting_window(&mut self) -> ArbResult {
        // Only operator can close window
        if msg::sender() != self.operator.get() {
            return Err(Vec::from(b"Not authorized"));
        }

        // Check window is active
        if !self.window_active.get() {
            return Err(Vec::from(b"No active window"));
        }

        // Deactivate window
        self.window_active.set(false);

        // Emit event
        evm::log(WindowClosed {
            operator: msg::sender(),
            timestamp: U256::from(block::timestamp()),
        });

        Ok(Vec::new())
    }

    pub fn get_nonce(&self, addr: Address) -> U256 {
        self.nonces.get(addr)
    }

    fn verify_signature(
        &self,
        bettor: Address,
        selected_address: Address,
        position: bool,
        amount: U256,
        nonce: U256,
        deadline: U256,
        signature: Bytes,
    ) -> Result<(), Vec<u8>> {
        // Check deadline
        if U256::from(block::timestamp()) > deadline {
            return Err(Vec::from(b"Signature expired"));
        }

        // Verify nonce
        let stored_nonce = self.nonces.get(bettor);
        if stored_nonce != nonce {
            return Err(Vec::from(b"Invalid nonce"));
        }

        // Manually pack parameters into bytes (packed encoding)
        let bettor_bytes = bettor.as_slice();
        let selected_address_bytes = selected_address.as_slice();
        let position_byte = if position { [1u8] } else { [0u8] };
        let amount_bytes: [u8; 32] = amount.to_be_bytes();
        let nonce_bytes: [u8; 32] = nonce.to_be_bytes();
        let deadline_bytes: [u8; 32] = deadline.to_be_bytes();

        // Manually pack parameters instead of using Solidity's abi.encodePacked()
        // to ensure byte-for-byte compatibility with our Rust backend's implementation.
        // TODO: Library encoding might add padding or use different struct packing rules.
        let mut message_data = Vec::new();
        message_data.extend_from_slice(bettor_bytes);
        message_data.extend_from_slice(selected_address_bytes);
        message_data.extend_from_slice(&position_byte);
        message_data.extend_from_slice(&amount_bytes);
        message_data.extend_from_slice(&nonce_bytes);
        message_data.extend_from_slice(&deadline_bytes);

        // Compute message hash and apply Ethereum Signed Message prefix
        let message_hash = keccak(message_data);
        let eth_signed_message_hash =
            keccak([SIGNED_MESSAGE_HEAD.as_bytes(), &message_hash.as_slice()].concat());

        // Recover signer using the prefixed hash
        let signer = self
            .recover_signer(eth_signed_message_hash, signature)
            .map_err(|_| Vec::from(b"Invalid signature"))?;

        if signer != bettor {
            return Err(Vec::from(b"Invalid signer"));
        }

        Ok(())
    }

    pub fn place_bet_with_signature(
        &mut self,
        bettor: Address,
        selected_address: Address,
        position: bool,
        amount: U256,
        nonce: U256,
        deadline: U256,
        signature: Bytes,
    ) -> ArbResult {
        self.verify_signature(
            bettor,
            selected_address,
            position,
            amount,
            nonce,
            deadline,
            signature,
        )?;

        // Increment nonce
        let new_nonce = self.nonces.get(bettor) + U256::from(1);
        self.nonces.insert(bettor, new_nonce);

        // Execute bet logic
        self.place_bet(bettor, selected_address, position, amount)
    }

    pub fn recover_signer(
        &self,
        eth_signed_message_hash: FixedBytes<32>,
        signature: Bytes,
    ) -> Result<Address, AddressLoggerError> {
        let (r, s, v) = self.split_signature(signature);
        self.ecrecover_call(eth_signed_message_hash, v, r, s)
    }

    /// Invoke the ECRECOVER precompile.
    pub fn ecrecover_call(
        &self,
        hash: FixedBytes<32>,
        v: u8,
        r: FixedBytes<32>,
        s: FixedBytes<32>,
    ) -> Result<Address, AddressLoggerError> {
        let data = (hash, v, r, s);
        let encoded_data = ECRECOVERType::abi_encode(&data);
        match call::static_call(Call::new(), ECRECOVER, &encoded_data) {
            Ok(result) => Ok(SOLAddress::abi_decode(&result, false).unwrap()),
            Err(_) => Err(AddressLoggerError::EcrecoverCallError(
                EcrecoverCallError {},
            )),
        }
    }

    fn split_signature(&self, signature: Bytes) -> (FixedBytes<32>, FixedBytes<32>, u8) {
        let r = FixedBytes::from_slice(&signature[0..32]);
        let s = FixedBytes::from_slice(&signature[32..64]);
        let v = signature[64];
        (r, s, v)
    }

    pub fn place_bet(
        &mut self,
        bettor: Address,
        selected_address: Address,
        position: bool,
        amount: U256,
    ) -> ArbResult {
        // Check window is active
        if !self.window_active.get() {
            return Err(Vec::from(b"No active betting window"));
        }

        // Check if selected address is valid
        let mut is_valid = false;
        for i in 0..self.valid_addresses.len() {
            if let Some(addr) = self.valid_addresses.getter(i) {
                if addr.get() == selected_address {
                    is_valid = true;
                    break;
                }
            }
        }
        if !is_valid {
            return Err(Vec::from(b"Invalid address selected"));
        }

        let token = IERC20::new(self.token_address.get());

        let allowance = token.allowance(Call::new_in(self), bettor, address())?;
        if allowance < amount {
            return Err(Vec::from(b"Insufficient allowance"));
        }
        // Calculate fee and bet amount
        let fee_amount = (amount * U256::from(FEE_PERCENTAGE)) / U256::from(100);
        let bet_amount = amount - fee_amount;
        let treasury_addr = self.treasury.get();

        // Transfer tokens from bettor to contract
        token.transfer_from(Call::new_in(self), bettor, address(), amount)?;

        // Transfer fee to treasury
        token.transfer(Call::new_in(self), treasury_addr, fee_amount)?;

        // Store bet
        let mut new_bet = self.bets.grow();
        new_bet.bettor.set(bettor);
        new_bet.selected_address.set(selected_address);
        new_bet.position.set(position);
        new_bet.amount.set(bet_amount);

        // Update total amounts for this address
        let mut addr_index = 0;
        for i in 0..self.valid_addresses.len() {
            if let Some(addr) = self.valid_addresses.getter(i) {
                if addr.get() == selected_address {
                    addr_index = i;
                    break;
                }
            }
        }

        if position {
            let mut amounts = self
                .address_up_amounts
                .setter(addr_index)
                .expect("no up amount");
            let get_amt = amounts.get();
            amounts.set(get_amt + bet_amount);
        } else {
            let mut amounts = self
                .address_down_amounts
                .setter(addr_index)
                .expect("no down amount");
            let get_amt = amounts.get();
            amounts.set(get_amt + bet_amount);
        }

        // Emit event
        evm::log(BetPlaced {
            bettor,
            selectedAddress: selected_address,
            position,
            amount: bet_amount,
        });

        Ok(Vec::new())
    }
    // Helper functions
    pub fn get_window_active(&self) -> bool {
        self.window_active.get()
    }

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

    pub fn process_payouts(&mut self, winners: Vec<bool>) -> Result<(), Vec<u8>> {
        // Only operator can process payouts
        if msg::sender() != self.operator.get() {
            return Err(Vec::from(b"Not authorized"));
        }

        // Validate winners array matches addresses
        if winners.len() != self.valid_addresses.len() {
            return Err(Vec::from(b"Invalid winners array length"));
        }

        let token = IERC20::new(self.token_address.get());
        let treasury_addr = self.treasury.get();

        // Process each address
        for i in 0..self.valid_addresses.len() {
            let up_amount = self.address_up_amounts.getter(i).unwrap().get();
            let down_amount = self.address_down_amounts.getter(i).unwrap().get();

            if up_amount == U256::ZERO || down_amount == U256::ZERO {
                // If either side has no bets, send all funds to treasury
                let total = up_amount + down_amount;
                if total > U256::ZERO {
                    token.transfer(Call::new_in(self), treasury_addr, total)?;
                }
                continue;
            }

            // Determine winning and losing pools
            let (winning_pool, losing_pool) = if winners[i] {
                (up_amount, down_amount)
            } else {
                (down_amount, up_amount)
            };

            // Collect all payouts first
            let mut payouts = Vec::new();
            for j in 0..self.bets.len() {
                if let Some(bet) = self.bets.getter(j) {
                    if bet.selected_address.get() == self.valid_addresses.getter(i).unwrap().get()
                        && bet.position.get() == winners[i]
                    {
                        let proportion = (bet.amount.get() * U256::from(1000000)) / winning_pool;
                        let winnings = (losing_pool * proportion) / U256::from(1000000);
                        let total_payout = bet.amount.get() + winnings;
                        payouts.push((bet.bettor.get(), total_payout));
                    }
                }
            }

            // Process payouts
            for (bettor, amount) in payouts {
                token.transfer(Call::new_in(self), bettor, amount)?;
                evm::log(PayoutProcessed {
                    bettor,
                    amount,
                    isWinner: true,
                });
            }
        }

        // Clear bets after processing
        let _len = self.bets.len();
        unsafe { self.bets.set_len(0) };

        Ok(())
    }

    pub fn is_valid_address(&self, address: Address) -> bool {
        for i in 0..self.valid_addresses.len() {
            if let Some(addr) = self.valid_addresses.getter(i) {
                if addr.get() == address {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_operator(&self) -> Address {
        self.operator.get()
    }

    pub fn get_treasury(&self) -> Address {
        self.treasury.get()
    }

    pub fn get_token(&self) -> Address {
        self.token_address.get()
    }
    pub fn get_up_amount(&self, addr_index: U256) -> Result<U256, Vec<u8>> {
        let idx = addr_index.as_limbs()[0] as usize;
        if idx >= self.valid_addresses.len() {
            return Err(Vec::from(b"Invalid address index"));
        }
        Ok(self.address_up_amounts.getter(addr_index).unwrap().get())
    }

    pub fn get_down_amount(&self, addr_index: U256) -> Result<U256, Vec<u8>> {
        let idx = addr_index.as_limbs()[0] as usize;
        if idx >= self.valid_addresses.len() {
            return Err(Vec::from(b"Invalid address index"));
        }
        Ok(self.address_down_amounts.getter(addr_index).unwrap().get())
    }
}
