use wallet::hd_wallet::HDWallet;
use wallet::signer::{schnorr_sign, ecdsa_sign, p2pkh_sign, p2tr_sign, Prevout};

uniffi_macros::include_scaffolding!("thing");
