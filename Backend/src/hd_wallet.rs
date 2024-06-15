extern crate bip32;
extern crate bitcoin;

use bip32::{ExtendedPrivateKey, DerivationPath, Language, Mnemonic, Seed};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use std::str::FromStr;

pub struct HDWallet {
    master_key: ExtendedPrivateKey,
    network: Network,
}

impl HDWallet {
    pub fn new(seed_phrase: &str, network: Network) -> Self {
        let mnemonic = Mnemonic::new(seed_phrase, Language::English).unwrap();
        let seed = Seed::new(&mnemonic);
        let master_key = ExtendedPrivateKey::new_master(network.into(), &seed).unwrap();

        HDWallet {
            master_key,
            network,
        }
    }

    pub fn derive_address(&self, derivation_path: &str) -> Address {
        let path = DerivationPath::from_str(derivation_path).unwrap();
        let derived_key = self.master_key.derive_priv(&path).unwrap();
        let private_key = PrivateKey {
            compressed: true,
            network: self.network,
            key: derived_key.private_key,
        };
        let public_key = private_key.public_key(&bitcoin::secp256k1::Secp256k1::new());
        Address::p2pkh(&public_key, self.network)
    }

    pub fn from_seed(seed_phrase: &str, network: Network) -> Self {
        Self::new(seed_phrase, network)
    }

    pub fn recover(seed_phrase: &str, network: Network) -> Self {
        Self::new(seed_phrase, network)
    }
}
