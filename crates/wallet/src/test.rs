use crate::hd_wallet::HDWallet;
use crate::signer::{ecdsa_sign, p2pkh_sign, p2tr_sign, schnorr_sign, Prevout};
use bitcoin::hashes::{sha256, Hash};
use bitcoin::hex::DisplayHex;
use bitcoin::key::TapTweak;
use bitcoin::script::ScriptBuf;
use bitcoin::secp256k1::{ecdsa, schnorr, Keypair, Message, Secp256k1, SecretKey};
use bitcoin::*;
use reqwest::Client;
use serde_json::json;
use std::str::FromStr;

const BTC_RPC_URL: &str = "http://127.0.0.1:18332";

#[test]
fn test_ecdsa_sign() {
    let message = "hello world";
    let priv_hex = "6cd9dc64451b6652203df996e255859aa9eefac8e99b9143510fafe5cae27822";
    let hash_hex = sha256::Hash::hash(message.as_bytes())
        .to_byte_array()
        .as_hex()
        .to_string();
    let sig = ecdsa_sign(priv_hex, &hash_hex);

    let signature = ecdsa::Signature::from_str(sig.as_str()).unwrap();
    let secp = Secp256k1::new();
    let private_key = SecretKey::from_str(priv_hex).unwrap();
    let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
    assert!(secp
        .verify_ecdsa(&msg, &signature, &private_key.public_key(&secp))
        .is_ok());
}

#[test]
fn test_schnorr_sign() {
    let message = "hello world";
    let merkle_root = None;
    let wallet = HDWallet::new(
        1,
        Some("visit frame clay clap often dance pair cousin peanut thumb fine foster".to_string()),
    );
    let tweaked_priv_hex = &wallet.bip86_tweaked_priv_hex(None);
    let hash_hex = sha256::Hash::hash(message.as_bytes())
        .to_byte_array()
        .as_hex()
        .to_string();
    let sig = schnorr_sign(tweaked_priv_hex, &hash_hex);

    let signature = schnorr::Signature::from_str(sig.as_str()).unwrap();
    let secp = Secp256k1::new();
    let keypair = Keypair::from_seckey_str(&secp, &wallet.bip86_priv_hex())
        .unwrap()
        .tap_tweak(&secp, merkle_root);
    let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
    assert!(secp
        .verify_schnorr(&signature, &msg, &keypair.to_inner().x_only_public_key().0)
        .is_ok());
}

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
    println!("mnemonic: {}", wallet3.export_mnemonic());
    println!("master extended key: {}", wallet3.export_master_priv());
    println!("evm private key: {}", wallet3.evm_priv_hex());
    println!("evm address: {}", wallet3.evm_address());
    println!("bip44 private key: {}", wallet3.bip44_priv_hex());
    println!("bip44 address: {}", wallet3.bip44_address());
    println!("bip86 private key: {}", wallet3.bip86_priv_hex());
    println!(
        "bip86 tweaked private key: {}",
        wallet3.bip86_tweaked_priv_hex(None)
    );
    println!("bip86 address: {}", wallet3.bip86_address());
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
