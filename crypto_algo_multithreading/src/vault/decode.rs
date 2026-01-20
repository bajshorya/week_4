use crate::crypto::aes::decrypt;
use std::{fs, path::PathBuf};

pub fn run(vault: PathBuf, seed: String, out_dir: PathBuf, _threads: usize) {
    let seed = hex::decode(seed).unwrap();
    fs::create_dir_all(&out_dir).unwrap();

    println!("[DECODE] Starting decryption process...\n");
    let root = fs::read_to_string(vault.join("root.commit")).unwrap();
    extract_node(&vault, &seed, root.trim(), &out_dir);
    println!("\n[DECODE] Decryption completed!");
}

fn extract_node(vault: &PathBuf, seed: &[u8], node_path: &str, out_dir: &PathBuf) {
    let enc = fs::read(vault.join("nodes").join(format!("{}.bin", node_path))).unwrap();
    println!("[DECODE] Decrypting node: {}", node_path);
    let data = decrypt(seed, node_path, &enc);

    if node_path.starts_with("leaf_") {
        fs::write(out_dir.join(format!("{}.bin", node_path)), data).unwrap();
        println!("[DECODE] Leaf extracted: {} -> {}.bin", node_path, node_path);
        return;
    }

    let content = String::from_utf8_lossy(&data);
    for part in content.split('|') {
        extract_node(vault, seed, part, out_dir);
    }
}
