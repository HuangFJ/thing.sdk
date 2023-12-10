use wasm_bindgen::prelude::*;
use wallet::hd_wallet;

#[wasm_bindgen]
pub struct HDWallet {
    inner: hd_wallet::HDWallet,
}

#[wasm_bindgen]
impl HDWallet {
    #[wasm_bindgen(constructor)]
    pub fn new(is_testnet: u8, mnemonic_str: Option<String>) -> Self {
        Self {
            inner: hd_wallet::HDWallet::new(is_testnet, mnemonic_str),
        }
    }

    pub fn from_master_priv(master_priv: &str) -> Self {
        Self {
            inner: hd_wallet::HDWallet::from_master_priv(master_priv),
        }
    }

    pub fn export_mnemonic(&self) -> String {
        self.inner.export_mnemonic()
    }

    pub fn export_master_priv(&self) -> String {
        self.inner.export_master_priv()
    }

    pub fn evm_address(&self) -> String {
        self.inner.evm_address()
    }

    pub fn evm_priv_hex(&self) -> String {
        self.inner.evm_priv_hex()
    }

    pub fn bip44_address(&self) -> String {
        self.inner.bip44_address()
    }
    pub fn bip44_priv_hex(&self) -> String {
        self.inner.bip44_priv_hex()
    }
}