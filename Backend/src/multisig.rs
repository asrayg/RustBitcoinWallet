extern crate bitcoin;

use bitcoin::util::address::Address;
use bitcoin::util::key::PublicKey;
use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::network::constants::Network;
use bitcoin::blockdata::script::Builder;

pub fn create_multisig_address(public_keys: Vec<PublicKey>, network: Network) -> Address {
    let script = Builder::new()
        .push_int(public_keys.len() as i64)
        .extend(public_keys.iter().map(|pk| pk.to_bytes()))
        .push_int(public_keys.len() as i64)
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKMULTISIG)
        .into_script();
    Address::p2sh(&script, network)
}

pub fn sign_multisig_transaction(
    psbt: &mut PartiallySignedTransaction,
    public_key: &PublicKey,
    private_key: &bitcoin::util::key::PrivateKey,
) {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let msg = psbt.global.unsigned_tx.signature_hash(0, &public_key.script_pubkey(), 1);
    let message = bitcoin::secp256k1::Message::from_slice(&msg[..]).unwrap();
    let sig = secp.sign(&message, &private_key.key);
    psbt.inputs[0].partial_sigs.insert(*public_key, sig);
}
