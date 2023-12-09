mod hd_wallet;
mod signer;

use hd_wallet::HDWallet;
use signer::{p2pkh_sign, p2tr_sign, Prevout};

pub struct ExampleCustomType<'a>(&'a str);

impl<'a> UniffiCustomTypeConverter for ExampleCustomType<'a> {
    type Builtin = &'a str;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Self(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

uniffi_macros::include_scaffolding!("thing");

#[cfg(test)]
mod test;
