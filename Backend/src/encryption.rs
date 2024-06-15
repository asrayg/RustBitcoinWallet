extern crate aes;
extern crate hex;

use aes::Aes256;
use aes::cipher::{BlockCipher, NewBlockCipher};
use aes::cipher::generic_array::GenericArray;

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut buffer = data.to_vec();
    cipher.encrypt_block(GenericArray::from_mut_slice(&mut buffer));
    buffer
}

pub fn decrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut buffer = data.to_vec();
    cipher.decrypt_block(GenericArray::from_mut_slice(&mut buffer));
    buffer
}
