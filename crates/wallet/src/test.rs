use std::str::FromStr;

use crate::hd_wallet::HDWallet;
use crate::signer::{p2pkh_sign, p2tr_sign, Prevout};
use bitcoin::hex::DisplayHex;
use bitcoin::script::ScriptBuf;
use bitcoin::*;
use reqwest::Client;
use serde_json::json;

#[test]
fn test_hd_wallet() {
    let wallet3 = HDWallet::new(
        1,
        Some("visit frame clay clap often dance pair cousin peanut thumb fine foster".to_string()),
    );
    assert_eq!(
        wallet3.export_mnemonic(),
        "visit frame clay clap often dance pair cousin peanut thumb fine foster"
    );
    assert_eq!(wallet3.export_master_priv() , "tprv8ZgxMBicQKsPdnUc4gyKBXrrp8Nq6gSSRV1yr3Rg6bsC4M1b19WFiAsD1b6ANibyTGVSY6D7JSYyhrv27EfvPMq99LQ847BbYK1uLf8wPpu");
    assert_eq!(
        wallet3.bip44_priv_hex(),
        "6cd9dc64451b6652203df996e255859aa9eefac8e99b9143510fafe5cae27822"
    );
    assert_eq!(
        wallet3.bip44_address(),
        "mzn7vdLThH2RRknmEMGZ8QB7tEQkmDaCWF"
    );
}

#[test]
fn test_evm() {
    let wallet = HDWallet::new(
        0,
        Some(
            "work man father plunge mystery proud hollow address reunion sauce theory bonus"
                .to_string(),
        ),
    );

    println!("private key: {}", wallet.evm_priv_hex());
    assert_eq!(
        wallet.evm_address(),
        "0xffDb339065c91c88e8a3cC6857359B6c2FB78cf5"
    );
}

#[tokio::test]
async fn test_p2tr_sign() {
    const BTC_RPC_URL: &str = "http://127.0.0.1:18332";
    // from
    const ADDRESS: &str = "tb1pakgwynt8cvc6wqeac3zxc3cpgkgcwdwyfehunlafyckcukq0h24q4p2kxa";
    const PRIV_HEX: &str = "6cd9dc64451b6652203df996e255859aa9eefac8e99b9143510fafe5cae27822";
    const INPUT_TXID: &str = "eaa5b43552c0fcde1a1126b7c6fb45089cba0377cbf1f1eeedc63d8b5adc4bfd";
    const INPUT_VOUT: u32 = 0;
    const INPUT_VALUE: f64 = 0.0001;
    // to
    const RECIPIENT: &str = "mzn7vdLThH2RRknmEMGZ8QB7tEQkmDaCWF";

    let input = TxIn {
        previous_output: OutPoint {
            txid: Txid::from_str(INPUT_TXID).unwrap(),
            vout: INPUT_VOUT,
        },
        script_sig: ScriptBuf::new(),
        sequence: Sequence(0),
        witness: Witness::new(),
    };

    let prevouts = vec![Prevout {
        txid: INPUT_TXID.to_string(),
        vout: INPUT_VOUT,
        amount: INPUT_VALUE,
    }];

    let tx = Transaction {
        version: transaction::Version::ONE,  // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![TxOut {
            value: Amount::from_btc(INPUT_VALUE)
                .unwrap()
                .checked_sub(Amount::from_sat(102))
                .unwrap(),
            script_pubkey: Address::from_str(RECIPIENT)
                .unwrap()
                .assume_checked()
                .script_pubkey(),
        }],
    };

    let tx_hex = p2tr_sign(
        ADDRESS,
        PRIV_HEX,
        consensus::serialize(&tx).as_hex().to_string().as_str(),
        prevouts,
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
    // from and to
    const ADDRESS: &str = "mzn7vdLThH2RRknmEMGZ8QB7tEQkmDaCWF";
    const PRIV_HEX: &str = "6cd9dc64451b6652203df996e255859aa9eefac8e99b9143510fafe5cae27822";
    const INPUT_TXID: &str = "3d1e955111f97c58a64d71215fb58de4a12eaea7b8f4fe95d771f35b708b0974";
    const INPUT_VOUT: u32 = 0;
    const INPUT_VALUE: f64 = 0.00009898;

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
            script_pubkey: Address::from_str(ADDRESS)
                .unwrap()
                .assume_checked()
                .script_pubkey(),
        }],
    };

    let tx_hex = p2pkh_sign(
        ADDRESS,
        PRIV_HEX,
        consensus::serialize(&tx).as_hex().to_string().as_str(),
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
