use bitcoin::{
    consensus, ecdsa,
    hashes::Hash,
    hashes::{hex::FromHex, sha256},
    hex::DisplayHex,
    key::{TapTweak, TweakedKeypair},
    script,
    secp256k1::{Keypair, Message, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    taproot, Address, Amount, EcdsaSighashType, OutPoint, PublicKey, TapSighashType, Transaction,
    TxOut, Txid,
};
use std::str::FromStr;

pub struct Prevout {
    pub txid: String,
    pub vout: u32,
    pub amount: f64,
}

/// ### Sign a tx with p2tr address
///
/// address:
/// bip86 address
///
/// priv_hex:
/// private key in hex, the tx inputs are locked by the p2tr address of this private key
///
/// tx_hex:
/// unsigned transaction in hex
///
/// tx_prevouts_json:
/// responding prevouts of tx inputs like [{"txid": "xxx", "vout": 0, "amount": 0.0001}, ...]
pub fn p2tr_sign(address: &str, priv_hex: &str, tx_hex: &str, tx_prevouts: Vec<Prevout>) -> String {
    let mut unsigned_tx =
        consensus::deserialize::<Transaction>(&Vec::<u8>::from_hex(tx_hex).unwrap()).unwrap();

    let secp = Secp256k1::new();
    let address = Address::from_str(address).unwrap().assume_checked();
    let network = address.network().clone();
    let private_key = SecretKey::from_str(priv_hex).unwrap();
    let private_addr = Address::p2tr(
        &secp,
        private_key.public_key(&secp).x_only_public_key().0,
        None,
        network,
    );

    assert_eq!(address, private_addr);

    let utxos = tx_prevouts
        .iter()
        .map(|v| {
            let txid = &v.txid;
            let vout = v.vout;
            let amount = v.amount;
            (
                OutPoint {
                    txid: Txid::from_str(txid).unwrap(),
                    vout,
                },
                TxOut {
                    value: Amount::from_btc(amount).unwrap(),
                    script_pubkey: private_addr.script_pubkey(),
                },
            )
        })
        .collect::<Vec<_>>();
    let txouts = utxos.into_iter().map(|(_, v)| v).collect::<Vec<TxOut>>();
    let prevouts = Prevouts::All(&txouts);

    let keypair = Keypair::from_secret_key(&secp, &private_key);
    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);

    let input_len = unsigned_tx.input.len();

    let hash_ty = TapSighashType::Default;
    let mut sighash_cache = SighashCache::new(&mut unsigned_tx);
    for i in 0..input_len {
        let sighash = sighash_cache
            .taproot_key_spend_signature_hash(i, &prevouts, hash_ty)
            .unwrap();

        let msg = Message::from_digest(sighash.to_byte_array());

        let signature = taproot::Signature {
            sig: secp.sign_schnorr(&msg, &tweaked.to_inner()),
            hash_ty,
        };

        sighash_cache
            .witness_mut(i)
            .unwrap()
            .push(signature.to_vec());
    }

    let tx = sighash_cache.into_transaction();
    let tx_hex = consensus::serialize(&tx).as_hex().to_string();

    tx_hex
}

/// ### Sign a tx with p2pkh address
///
/// address:
/// bip44 address
///
/// priv_hex:
/// private key in hex, the tx inputs are locked by the p2pkh address of this private key
///
/// tx_hex:
/// unsigned transaction in hex
pub fn p2pkh_sign(address: &str, priv_hex: &str, tx_hex: &str) -> String {
    let mut unsigned_tx =
        consensus::deserialize::<Transaction>(&Vec::<u8>::from_hex(tx_hex).unwrap()).unwrap();

    let secp = Secp256k1::new();
    let address = Address::from_str(address).unwrap().assume_checked();
    let network = address.network().clone();
    let private_key = SecretKey::from_str(priv_hex).unwrap();
    let pubkey = PublicKey::new(private_key.public_key(&secp));
    let private_addr = Address::p2pkh(&pubkey, network);

    assert_eq!(address, private_addr);

    let input_len = unsigned_tx.input.len();

    let hash_ty = EcdsaSighashType::All;
    let sighash_cache = SighashCache::new(&mut unsigned_tx);
    let mut script_sigs = Vec::new();
    for i in 0..input_len {
        let sighash = sighash_cache
            .legacy_signature_hash(
                i,
                private_addr.script_pubkey().as_script(),
                hash_ty.to_u32(),
            )
            .unwrap();

        let msg = Message::from_digest(sighash.to_byte_array());

        let signature = ecdsa::Signature {
            sig: secp.sign_ecdsa(&msg, &private_key),
            hash_ty,
        }
        .serialize();

        let mut push_bytes = script::PushBytesBuf::new();
        push_bytes.extend_from_slice(signature.as_ref()).unwrap();

        let script_sig = script::Builder::new()
            .push_slice(&push_bytes)
            .push_key(&pubkey)
            .into_script();
        script_sigs.push(script_sig);
    }
    let tx = sighash_cache.into_transaction();
    for i in 0..input_len {
        tx.input[i].script_sig = script_sigs[i].clone();
    }
    let tx_hex = consensus::serialize(&tx).as_hex().to_string();

    tx_hex
}

pub fn ecdsa_sign(priv_hex: &str, message: &str) -> String {
    let secp = Secp256k1::new();
    let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
    let private_key = SecretKey::from_str(priv_hex).unwrap();
    secp.sign_ecdsa(&msg, &private_key)
        .serialize_der()
        .as_hex()
        .to_string()
}

pub fn schnorr_sign(tweaked_priv_hex: &str, message: &str) -> String {
    let secp = Secp256k1::new();
    let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
    let keypair = Keypair::from_seckey_str(&secp, tweaked_priv_hex).unwrap();
    secp.sign_schnorr(&msg, &keypair)
        .serialize()
        .as_hex()
        .to_string()
}
