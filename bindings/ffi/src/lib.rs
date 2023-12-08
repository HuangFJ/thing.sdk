mod hd_wallet;
mod signer;

use hd_wallet::HDWallet;
use signer::{p2pkh_sign, p2tr_sign};

uniffi_macros::include_scaffolding!("thing");

#[cfg(test)]
mod test;