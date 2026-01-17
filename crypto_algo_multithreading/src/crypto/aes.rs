use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};

use super::kdf::derive_key;

pub fn encrypt(seed: &[u8], path: &str, data: &[u8]) -> Vec<u8> {
    let key = derive_key(seed, path);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let nonce = Nonce::from_slice(&[0u8; 12]);
    cipher.encrypt(nonce, data).unwrap()
}

pub fn decrypt(seed: &[u8], path: &str, data: &[u8]) -> Vec<u8> {
    let key = derive_key(seed, path);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let nonce = Nonce::from_slice(&[0u8; 12]);
    cipher.decrypt(nonce, data).unwrap()
}
