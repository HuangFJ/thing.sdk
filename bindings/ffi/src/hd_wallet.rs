use std::str::FromStr;

use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::hashes::hex::FromHex;
use bitcoin::secp256k1::{All, Secp256k1};
use bitcoin::{Address, PublicKey};

pub struct HDWallet {
    root: Xpriv,
    secp: Secp256k1<All>,
}

impl HDWallet {
    pub fn new(coin_type: u8, seed_hex: String) -> Self {
        let network = match coin_type {
            0 => bitcoin::Network::Bitcoin,
            1 => bitcoin::Network::Testnet,
            _ => unreachable!(),
        };
        let seed = Vec::from_hex(seed_hex.as_str()).unwrap();
        let root = Xpriv::new_master(network, &seed).unwrap();

        let secp = Secp256k1::new();

        Self { root, secp }
    }

    pub fn from_master_priv(master_priv: String) -> Self{
        let root = Xpriv::from_str(master_priv.as_str()).unwrap();
        let secp = Secp256k1::new();
        Self { root, secp }
    }

    pub fn bip44_address(&self) -> String {
        let coin_type = match self.root.network {
            bitcoin::Network::Bitcoin => "0'",
            bitcoin::Network::Testnet => "1'",
            _ => unreachable!(),
        };

        let path = DerivationPath::from_str(&format!("m/44'/{coin_type}/0'/0/0")).unwrap();
        let extended_prikey = self.root.derive_priv(&self.secp, &path).unwrap();

        let xpub = Xpub::from_priv(&self.secp, &extended_prikey);
        let pubkey = PublicKey::new(xpub.public_key);
        let address = Address::p2pkh(&pubkey, self.root.network);

        address.to_string()
    }

    pub fn bip86_address(&self) -> String {
        let coin_type = match self.root.network {
            bitcoin::Network::Bitcoin => "0'",
            bitcoin::Network::Testnet => "1'",
            _ => unreachable!(),
        };

        let path = DerivationPath::from_str(&format!("m/86'/{coin_type}/0'/0/0")).unwrap();
        let extended_prikey = self.root.derive_priv(&self.secp, &path).unwrap();
        let public_key = Xpub::from_priv(&self.secp, &extended_prikey).public_key;

        let address = Address::p2tr(
            &self.secp,
            public_key.x_only_public_key().0,
            None,
            self.root.network,
        );

        address.to_string()
    }
    
    pub fn export_master_priv(&self) -> String {
        self.root.to_string()
    }
}
