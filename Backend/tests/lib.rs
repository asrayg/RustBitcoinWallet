#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::network::constants::Network;
    use bitcoin::util::key::{PublicKey, PrivateKey};
    use secp256k1::Secp256k1;
    use secp256k1::rand::rngs::OsRng;

    #[test]
    fn test_hd_wallet_creation() {
        let seed_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let hd_wallet = HDWallet::new(seed_phrase, network);
        assert!(hd_wallet.master_key.network == network);
    }

    #[test]
    fn test_transaction_history() {
        let mut history = TransactionHistory::new("test_history.json");
        history.add_record("test_txid".to_string(), 1000);
        assert!(history.get_records().len() > 0);
    }

    #[test]
    fn test_qr_code_generation() {
        let address = "1BitcoinAddress...";
        let qr_code = generate_qr_code(address);
        save_qr_code(&qr_code, "test_qr_code.png");
        assert!(std::path::Path::new("test_qr_code.png").exists());
    }

    #[test]
    fn test_multisig_address_creation() {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().unwrap();
        let sk1 = PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: bitcoin::util::key::SecretKey::new(&mut rng),
        };
        let sk2 = PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: bitcoin::util::key::SecretKey::new(&mut rng),
        };
        let pk1 = PublicKey::from_private_key(&secp, &sk1);
        let pk2 = PublicKey::from_private_key(&secp, &sk2);
        let address = create_multisig_address(vec![pk1, pk2], Network::Bitcoin);
        assert!(address.is_valid());
    }
}
