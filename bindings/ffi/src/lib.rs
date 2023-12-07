mod hd_wallet;
mod signer;

use hd_wallet::HDWallet;
use signer::taproot_sign;

uniffi_macros::include_scaffolding!("thing");
