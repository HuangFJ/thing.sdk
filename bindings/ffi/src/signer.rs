use std::str::FromStr;

use bitcoin::hashes::hex::FromHex;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{
    consensus::deserialize,
    hashes::Hash,
    hex::DisplayHex,
    key::{TapTweak, TweakedKeypair},
    secp256k1::{Keypair, Message, SecretKey},
    sighash::SighashCache,
    TxOut,
};
use bitcoin::{Address, Amount, Transaction};

pub fn taproot_sign(
    priv_hex: String,
    tx_hex: String,
    tx_prevouts_json: String,
) -> String {
    let mut commit_tx =
        deserialize::<Transaction>(&Vec::<u8>::from_hex(tx_hex.as_str()).unwrap()).unwrap();

    let secp = Secp256k1::new();
    let utxos = serde_json::from_str::<Vec<serde_json::Value>>(tx_prevouts_json.as_str())
        .unwrap()
        .iter()
        .map(|v| {
            let txid = v["txid"].as_str().unwrap();
            let vout = v["vout"].as_u64().unwrap() as u32;
            let amount = v["amount"].as_f64().unwrap();
            let script_pubkey_hex = v["scriptPubKey"]["hex"].as_str().unwrap();
            (
                bitcoin::OutPoint {
                    txid: bitcoin::Txid::from_str(txid).unwrap(),
                    vout,
                },
                bitcoin::TxOut {
                    value: Amount::from_btc(amount).unwrap(),
                    script_pubkey: user_addr.script_pubkey(),
                },
            )
        })
        .collect::<Vec<_>>();
    let txouts = utxos.into_iter().map(|(_, v)| v).collect::<Vec<TxOut>>();
    let prevouts = bitcoin::sighash::Prevouts::All(&txouts);
    let private_key = SecretKey::from_str(priv_hex.as_str()).unwrap();
    let keypair = Keypair::from_secret_key(&secp, &private_key);
    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);

    let input_len = commit_tx.input.len();

    let hash_ty = bitcoin::TapSighashType::Default;
    let mut sighash_cache = SighashCache::new(&mut commit_tx);
    for i in 0..input_len {
        let sighash = sighash_cache
            .taproot_key_spend_signature_hash(i, &prevouts, hash_ty)
            .unwrap();

        let msg = Message::from_digest(sighash.to_byte_array());

        let signature = bitcoin::taproot::Signature {
            sig: secp.sign_schnorr(&msg, &tweaked.to_inner()),
            hash_ty,
        };

        sighash_cache
            .witness_mut(i)
            .unwrap()
            .push(signature.to_vec());
    }

    let tx = sighash_cache.into_transaction();
    let tx_hex = bitcoin::consensus::encode::serialize(&tx)
        .as_hex()
        .to_string();

    tx_hex
}
