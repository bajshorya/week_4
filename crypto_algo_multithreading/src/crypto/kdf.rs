use hkdf::Hkdf;
use sha2::Sha256;

pub fn derive_key(seed: &[u8], path: &str) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(None, seed);
    let mut key = [0u8; 32];
    hk.expand(path.as_bytes(), &mut key).unwrap();
    key
}
