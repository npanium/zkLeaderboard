use e2e::{
    alloy::primitives::{address, utils::parse_ether, U256},
    eyre::Result,
    send, tokio, Account, ReceiptExt,
};

use abi::AddressLogger::{self};
mod abi;

// Test Deployment
#[e2e::test]
async fn test_contract_deploys(alice: Account) -> Result<()> {
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = AddressLogger::new(contract_addr, &alice.wallet);

    // Verify initial state
    let ret = contract.getBetCount().call().await?;
    assert_eq!(ret._0, U256::ZERO);

    // Test we can call other functions
    let test_address = address!("d8da6bf26964af9d7eed9e03e53415d37aa96045");
    let _ = send!(contract.logAddresses(vec![test_address]));

    Ok(())
}

// Test functions
#[e2e::test]
async fn test_full_betting_flow(alice: Account, bob: Account) -> Result<()> {
    // Deploy contract
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = AddressLogger::new(contract_addr, &alice.wallet);

    // Test addresses
    let eligible_address = address!("d8da6bf26964af9d7eed9e03e53415d37aa96045");
    let addresses = vec![eligible_address];

    // Log addresses
    let _ = send!(contract.logAddresses(addresses.clone()));

    // Place bet from Bob
    let bet_amount = parse_ether("1")?;
    let contract_bob = AddressLogger::new(contract_addr, &bob.wallet);
    let _ = send!(contract_bob
        .placeBet(eligible_address, true)
        .value(bet_amount));

    // Verify bet was recorded
    let ret = contract.getBet(U256::ZERO).call().await?;
    assert_eq!(ret._0, bob.address()); // bettor
    assert_eq!(ret._1, eligible_address); // selected address
    assert_eq!(ret._2, true); // position
    assert_eq!(ret._3, bet_amount); // amount

    // Verify bet count
    let ret = contract.getBetCount().call().await?;
    assert_eq!(ret._0, U256::from(1));

    Ok(())
}

#[e2e::test]
async fn test_multiple_bets(alice: Account, bob: Account) -> Result<()> {
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = AddressLogger::new(contract_addr, &alice.wallet);

    // Log multiple eligible addresses
    let addresses = vec![
        address!("d8da6bf26964af9d7eed9e03e53415d37aa96045"),
        address!("71c7656ec7ab88b098defb751b7401b5f6d8976f"),
    ];
    let _ = send!(contract.logAddresses(addresses.clone()));

    // Place bets from different accounts
    let bet_amount = parse_ether("0.5")?;
    let contract_bob = AddressLogger::new(contract_addr, &bob.wallet);

    // Bob bets on first address
    let _ = send!(contract_bob.placeBet(addresses[0], true).value(bet_amount));

    // Alice bets on second address
    let _ = send!(contract.placeBet(addresses[1], false).value(bet_amount));

    // Test multiple bets
    let ret = contract.getBetCount().call().await?;
    assert_eq!(ret._0, U256::from(2));

    // Verify first bet
    let ret1 = contract.getBet(U256::ZERO).call().await?;
    assert_eq!(ret1._0, bob.address());
    assert_eq!(ret1._1, addresses[0]);
    assert!(ret1._2);
    assert_eq!(ret1._3, bet_amount);

    // Verify second bet
    let ret2 = contract.getBet(U256::from(1)).call().await?;
    assert_eq!(ret2._0, alice.address());
    assert_eq!(ret2._1, addresses[1]);
    assert!(!ret2._2);
    assert_eq!(ret2._3, bet_amount);

    Ok(())
}
