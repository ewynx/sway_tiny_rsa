use fuels::prelude::*;
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Tiny_RSA, "out/debug/rsa_sway-abi.json");

async fn get_contract_instance() -> Tiny_RSA{
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_single_wallet().await;

    let id = Contract::deploy("./out/debug/rsa_sway.bin", &wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = Tiny_RSA::new(id.to_string(), wallet);

    instance
}

/**
The tiny example comes from https://ccom.uprrp.edu/~humberto/very-small-rsa-example.html
**/

#[tokio::test]
async fn test_encrypt() {
    let _instance = get_contract_instance().await;
    let test = _instance.key_gen().call().await.unwrap();
    assert_eq!(test.value, (1,1));
    let m1: u64 = 10;
    let m2: u64 = 11;
    let c1 = _instance.encrypt(m1).call().await.unwrap();
    let c2 = _instance.encrypt(m2).call().await.unwrap();
    assert_eq!(c1.value, 117);
    assert_eq!(c2.value, 121);
}

#[tokio::test]
async fn test_decrypt() {
    let _instance = get_contract_instance().await;
    let m1: u64 = 10;
    let m2: u64 = 11;
    let c1 = _instance.encrypt(m1).call().await.unwrap();
    let c2 = _instance.encrypt(m2).call().await.unwrap();
    let _m1 = _instance.decrypt(c1.value).call().await.unwrap();
    let _m2 = _instance.decrypt(c2.value).call().await.unwrap();
    assert_eq!(m1, _m1.value);
    assert_eq!(m2, _m2.value);
}
