use wallet::hd_wallet;
use wallet::signer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Prevout {
    inner: signer::Prevout,
}

#[wasm_bindgen]
pub fn ecdsa_sign(priv_hex: &str, message: &str) -> String {
    signer::ecdsa_sign(priv_hex, message)
}

#[wasm_bindgen]
pub fn p2pkh_sign(address: &str, priv_hex: &str, tx_hex: &str) -> String {
    signer::p2pkh_sign(address, priv_hex, tx_hex)
}

#[wasm_bindgen]
pub fn p2tr_sign(address: &str, priv_hex: &str, tx_hex: &str, tx_prevouts: Vec<Prevout>) -> String {
    signer::p2tr_sign(
        address,
        priv_hex,
        tx_hex,
        tx_prevouts
            .into_iter()
            .map(|prevout| prevout.inner)
            .collect(),
    )
}

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

    pub fn bip44_address(&self) -> String {
        self.inner.bip44_address()
    }

    pub fn bip86_address(&self) -> String {
        self.inner.bip86_address()
    }

    pub fn evm_priv_hex(&self) -> String {
        self.inner.evm_priv_hex()
    }

    pub fn bip44_priv_hex(&self) -> String {
        self.inner.bip44_priv_hex()
    }

    pub fn bip86_priv_hex(&self) -> String {
        self.inner.bip86_priv_hex()
    }
}
