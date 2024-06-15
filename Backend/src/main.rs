mod encryption;
mod hd_wallet;
mod transactions;
mod qrcode;
mod multisig;
mod transaction_history;
mod watch_only;

use encryption::{decrypt, encrypt};
use hd_wallet::HDWallet;
use transactions::{create_transaction, fetch_utxos, broadcast_transaction};
use qrcode::{generate_qr_code, save_qr_code};
use multisig::{create_multisig_address, sign_multisig_transaction};
use transaction_history::TransactionHistory;
use watch_only::WatchOnlyWallet;

use bitcoin::network::constants::Network;
use bitcoin::util::amount::Amount;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use bitcoin::secp256k1::Secp256k1;
use rand::rngs::OsRng;
use std::str::FromStr;
use tokio;

#[tokio::main]
async fn main() {
    let seed_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"; // Example seed phrase
    let network = Network::Bitcoin;

    // Create and recover an HD Wallet
    let hd_wallet = HDWallet::new(seed_phrase, network);
    let recovery_wallet = HDWallet::recover(seed_phrase, network);

    // Derive an address from the HD Wallet
    let derivation_path = "m/44'/0'/0'/0/0";
    let address = hd_wallet.derive_address(derivation_path);
    println!("Derived Address: {}", address);

    // Fetch UTXOs for the address
    let utxos = fetch_utxos(&address).await;
    for utxo in &utxos {
        println!("UTXO: {:?}", utxo);
    }

    // Create a QR code for the address
    let qr_code = generate_qr_code(&address.to_string());
    save_qr_code(&qr_code, "address_qr_code.png");
    println!("QR code saved as address_qr_code.png");

    // Create a transaction (example usage)
    if let Some(utxo) = utxos.first() {
        let txid = utxo["tx_hash"].as_str().unwrap();
        let vout = utxo["tx_output_n"].as_u64().unwrap() as u32;
        let amount = Amount::from_sat(utxo["value"].as_u64().unwrap());

        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let sk = PrivateKey {
            compressed: true,
            network: network,
            key: secp256k1::SecretKey::new(&mut rng),
        };

        let outputs = vec![(Address::from_str("recipient_address").unwrap(), amount)];
        let transaction = create_transaction(vec![(txid.to_string(), vout)], outputs, &sk);
        broadcast_transaction(&transaction).await;
        println!("Transaction broadcasted");

        // Add transaction to history
        let mut history = TransactionHistory::new("transaction_history.json");
        history.add_record(txid.to_string(), amount.as_sat() as i64);
        println!("Transaction added to history");
    }

    // Multi-sig address creation
    let mut rng = OsRng;
    let sk1 = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        key: secp256k1::SecretKey::new(&mut rng),
    };
    let sk2 = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        key: secp256k1::SecretKey::new(&mut rng),
    };
    let pk1 = sk1.public_key(&Secp256k1::new());
    let pk2 = sk2.public_key(&Secp256k1::new());
    let multisig_address = create_multisig_address(vec![pk1, pk2], Network::Bitcoin);
    println!("Multi-sig Address: {}", multisig_address);

    // Watch-only wallet
    let mut watch_wallet = WatchOnlyWallet::new();
    watch_wallet.add_address(address.clone());
    if let Err(e) = watch_wallet.fetch_utxos().await {
        println!("Error fetching UTXOs: {}", e);
    }
}
