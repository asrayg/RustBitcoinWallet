extern crate bitcoin;
extern crate reqwest;
extern crate serde_json;

use bitcoin::util::address::Address;
use serde_json::Value;
use std::error::Error;

pub struct WatchOnlyWallet {
    addresses: Vec<Address>,
}

impl WatchOnlyWallet {
    pub fn new() -> Self {
        WatchOnlyWallet { addresses: vec![] }
    }

    pub fn add_address(&mut self, address: Address) {
        self.addresses.push(address);
    }

    pub fn get_addresses(&self) -> &Vec<Address> {
        &self.addresses
    }

    pub async fn fetch_utxos(&self) -> Result<(), Box<dyn Error>> {
        for address in &self.addresses {
            println!("Fetching UTXOs for: {}", address);
            let url = format!("https://blockchain.info/unspent?active={}", address);
            let response = reqwest::get(&url).await?;
            let json: Value = response.json().await?;
            if let Some(utxos) = json["unspent_outputs"].as_array() {
                for utxo in utxos {
                    println!("UTXO: {:?}", utxo);
                }
            } else {
                println!("No UTXOs found for address: {}", address);
            }
        }
        Ok(())
    }
}
