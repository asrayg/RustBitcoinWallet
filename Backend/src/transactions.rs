extern crate bitcoin;
extern crate reqwest;
extern crate serde_json;

use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut};
use bitcoin::consensus::encode;
use bitcoin::util::address::Address;
use bitcoin::util::amount::Amount;
use bitcoin::util::key::PrivateKey;
use bitcoin::util::psbt::serialize::Deserialize;
use reqwest::Client;
use serde_json::Value;

pub async fn fetch_utxos(address: &Address) -> Vec<Value> {
    let url = format!("https://blockchain.info/unspent?active={}", address);
    let response = Client::new().get(&url).send().await.unwrap();
    let json: Value = response.json().await.unwrap();
    json["unspent_outputs"].as_array().unwrap().clone()
}

pub fn create_transaction(
    inputs: Vec<(String, u32)>,
    outputs: Vec<(Address, Amount)>,
    private_key: &PrivateKey,
) -> Transaction {
    let tx_ins: Vec<TxIn> = inputs
        .iter()
        .map(|(txid, _vout)| TxIn {
            previous_output: encode::deserialize(&hex::decode(txid).unwrap()).unwrap(),
            script_sig: Default::default(),
            sequence: 0xffff_ffff,
            witness: vec![],
        })
        .collect();

    let tx_outs: Vec<TxOut> = outputs
        .iter()
        .map(|(address, amount)| TxOut {
            value: amount.as_sat(),
            script_pubkey: address.script_pubkey(),
        })
        .collect();

    let mut transaction = Transaction {
        version: 2,
        lock_time: 0,
        input: tx_ins,
        output: tx_outs,
    };

    let secp = bitcoin::secp256k1::Secp256k1::new();
    for (i, txin) in transaction.input.iter_mut().enumerate() {
        let script_code = txin.script_sig.clone();
        let sighash = transaction.signature_hash(i, &script_code, bitcoin::blockdata::transaction::SigHashType::All.as_u32());
        let message = bitcoin::secp256k1::Message::from_slice(&sighash[..]).unwrap();
        let sig = secp.sign(&message, &private_key.key);
        let der_sig = sig.serialize_der();
        let mut final_sig = Vec::with_capacity(der_sig.len() + 1);
        final_sig.extend_from_slice(&der_sig);
        final_sig.push(bitcoin::blockdata::transaction::SigHashType::All as u8);
        txin.script_sig = bitcoin::blockdata::script::Builder::new().push_slice(&final_sig).into_script();
    }

    transaction
}

pub async fn broadcast_transaction(transaction: &Transaction) {
    let client = Client::new();
    let tx_hex = encode::serialize_hex(transaction);
    let url = "https://blockchain.info/pushtx";
    let _response = client.post(url).body(format!("tx={}", tx_hex)).send().await.unwrap();
}
