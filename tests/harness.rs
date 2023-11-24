use fuels::{
    macros::abigen,
    programs::contract::{Contract, LoadConfiguration},
    test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig},
    types::transaction::TxParameters,
};

abigen!(Contract(
    name = "TestContract",
    abi = "out/debug/test-contract-abi.json"
));

#[tokio::test]
async fn main_test() {
    let config = WalletsConfig::new(Some(1), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(config, None, None).await;
    let wallet = &wallets[0];

    let config = LoadConfiguration::default();
    let id = Contract::load_from("out/debug/test-contract.bin", config)
        .unwrap()
        .deploy(wallet, TxParameters::default().with_gas_price(1))
        .await
        .unwrap();

    let instance = TestContract::new(id, wallet.clone());

    let counter = instance
        .methods()
        .get_counter()
        .simulate()
        .await
        .unwrap()
        .value;
    println!("counter: {counter}");

    instance
        .methods()
        .increment_counter(1)
        .call()
        .await
        .unwrap();

    let counter = instance
        .methods()
        .get_counter()
        .simulate()
        .await
        .unwrap()
        .value;
    println!("counter: {counter}");

    let counter = instance
        .methods()
        .increment_counter(1)
        .call()
        .await
        .unwrap()
        .value;
    println!("counter: {counter}");
    
    let counter = instance
        .methods()
        .increment_counter(1)
        .call()
        .await
        .unwrap()
        .value;
    println!("counter: {counter}");
}
