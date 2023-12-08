use std::str::FromStr;

use crate::hd_wallet::HDWallet;
use crate::signer::{p2tr_sign, p2pkh_sign};
use bitcoin::*;
use bitcoin::hex::DisplayHex;
use bitcoin::script::ScriptBuf;
use reqwest::Client;
use serde_json::json;


#[test]
fn test_hd_wallet(){
    let wallet3 = HDWallet::new(1, "e9bc5fd1c14cbe5449e250596b4ffe655f84d9b8175d36c18a6b54d421aad2561cc7558aca1584034c460d2f66f7865395b7c86f24632157bb8f20737014ae6a".to_string());
    println!("{}", wallet3.bip44_priv_hex());
    println!("{}", wallet3.bip44_address());
}

#[tokio::test]
async fn test_p2tr_sign() {
    const BTC_RPC_URL: &str = "http://127.0.0.1:18332";
    const COIN_TYPE: u8 = 1;
    // [{"txid":"eaa5b43552c0fcde1a1126b7c6fb45089cba0377cbf1f1eeedc63d8b5adc4bfd","vout":0,"amount":0.0001}]
    const INPUT_TXID: &str = "eaa5b43552c0fcde1a1126b7c6fb45089cba0377cbf1f1eeedc63d8b5adc4bfd";
    const INPUT_VOUT: u32 = 0;
    const INPUT_VALUE: f64 = 0.0001;

    // tb1pakgwynt8cvc6wqeac3zxc3cpgkgcwdwyfehunlafyckcukq0h24q4p2kxa
    let priv_hex = "6cd9dc64451b6652203df996e255859aa9eefac8e99b9143510fafe5cae27822".to_string();

    let wallet = HDWallet::new(COIN_TYPE, "e9bc5fd1c14cbe5449e250596b4ffe655f84d9b8175d36c18a6b54d421aad2561cc7558aca1584034c460d2f66f7865395b7c86f24632157bb8f20737014ae6a".to_string());
    // mzn7vdLThH2RRknmEMGZ8QB7tEQkmDaCWF
    let recipient = wallet.bip44_address();

    let input = TxIn {
        previous_output: OutPoint {
            txid: Txid::from_str(INPUT_TXID).unwrap(),
            vout: INPUT_VOUT,
        },
        script_sig: ScriptBuf::new(),
        sequence: Sequence(0),
        witness: Witness::new(),
    };

    let prevouts = json!([
        {
            "txid": INPUT_TXID,
            "vout": INPUT_VOUT,
            "amount": INPUT_VALUE
        }
    ]);

    let tx = Transaction {
        version: transaction::Version::ONE,  // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![TxOut {
            value: Amount::from_btc(INPUT_VALUE)
                .unwrap()
                .checked_sub(Amount::from_sat(102))
                .unwrap(),
            script_pubkey: Address::from_str(&recipient)
                .unwrap()
                .assume_checked()
                .script_pubkey(),
        }],
    };

    let tx_hex = p2tr_sign(
        COIN_TYPE,
        priv_hex,
        consensus::serialize(&tx).as_hex().to_string(),
        serde_json::to_string(&prevouts).unwrap(),
    );

    let client = Client::new();
    let res: serde_json::Value = client
        .post(BTC_RPC_URL)
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "curltest",
            "method": "sendrawtransaction",
            "params": [tx_hex]
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("broadcast tx: {:?}", res);
}

#[tokio::test]
async fn test_p2pkh_sign() {
    const BTC_RPC_URL: &str = "http://127.0.0.1:18332";
    const COIN_TYPE: u8 = 1;
    const SEED_HEX: &str = "e9bc5fd1c14cbe5449e250596b4ffe655f84d9b8175d36c18a6b54d421aad2561cc7558aca1584034c460d2f66f7865395b7c86f24632157bb8f20737014ae6a";
    const INPUT_TXID: &str = "3d1e955111f97c58a64d71215fb58de4a12eaea7b8f4fe95d771f35b708b0974";
    const INPUT_VOUT: u32 = 0;
    const INPUT_VALUE: f64 = 0.00009898;

    let wallet = HDWallet::new(COIN_TYPE, SEED_HEX.to_string());
    // 6cd9dc64451b6652203df996e255859aa9eefac8e99b9143510fafe5cae27822
    let priv_hex = wallet.bip44_priv_hex();
    // mzn7vdLThH2RRknmEMGZ8QB7tEQkmDaCWF
    let addr = wallet.bip44_address();

    let input = TxIn {
        previous_output: OutPoint {
            txid: Txid::from_str(INPUT_TXID).unwrap(),
            vout: INPUT_VOUT,
        },
        script_sig: ScriptBuf::new(),
        sequence: Sequence(0),
        witness: Witness::new(),
    };

    let tx = Transaction {
        version: transaction::Version::ONE,  // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![TxOut {
            value: Amount::from_btc(INPUT_VALUE)
                .unwrap()
                .checked_sub(Amount::from_sat(192))
                .unwrap(),
            script_pubkey: Address::from_str(&addr)
                .unwrap()
                .assume_checked()
                .script_pubkey(),
        }],
    };

    let tx_hex = p2pkh_sign(
        COIN_TYPE,
        priv_hex,
        consensus::serialize(&tx).as_hex().to_string(),
    );

    let client = Client::new();
    let res: serde_json::Value = client
        .post(BTC_RPC_URL)
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "curltest",
            "method": "sendrawtransaction",
            "params": [tx_hex]
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("broadcast tx: {:?}", res);
}

