use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use clap::{Parser, Subcommand};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]   
enum Commands {
    Encode {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        threads: usize,
    },
    Decode {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        seed: String,
        #[arg(long)]
        out_dir: PathBuf, 
        #[arg(long)]
        threads: usize,
    },
}

fn derive_key(seed: &[u8], path: &str) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(None, seed);
    let mut key = [0u8; 32];
    hk.expand(path.as_bytes(), &mut key).unwrap();
    key
}

fn encrypt(seed: &[u8], path: &str, data: &[u8]) -> Vec<u8> {
    let key = derive_key(seed, path);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let nonce = Nonce::from_slice(&[0u8; 12]); // deterministic
    cipher.encrypt(nonce, data).unwrap()
}

fn decrypt(seed: &[u8], path: &str, data: &[u8]) -> Vec<u8> {
    let key = derive_key(seed, path);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let nonce = Nonce::from_slice(&[0u8; 12]);
    cipher.decrypt(nonce, data).unwrap()
}

fn extract_node(vault: &PathBuf, seed: &[u8], node_path: &str, out_dir: &PathBuf) -> Vec<String> {
    let enc_path = vault.join("nodes").join(format!("{}.bin", node_path));
    let enc = fs::read(&enc_path).unwrap();
    let data = decrypt(seed, node_path, &enc);
    let content = String::from_utf8_lossy(&data).to_string();

    if node_path.starts_with("leaf_") {
        let file_name = format!("file_{}.bin", node_path.strip_prefix("leaf_").unwrap());
        let output_path = out_dir.join(file_name);
        fs::write(output_path, &data).unwrap();
        println!("Extracted: {}", node_path);
        vec![content]
    } else {
        let parts: Vec<&str> = content.split('|').collect();
        let mut results = Vec::new();

        for part in parts {
            if !part.is_empty() && part != node_path {
                results.extend(extract_node(vault, seed, part, out_dir));
            }
        }
        results
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            files,
            out,
            threads,
        } => {
            fs::create_dir_all(out.join("nodes")).unwrap();

            let mut seed = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut seed);
            println!("Seed (save this!): {}", hex::encode(seed));
            println!("Encoding {} files using {} threads", files.len(), threads);

            let seed = Arc::new(seed);
            let files = Arc::new(files);
            let out_dir = Arc::new(out.clone());

            let mut handles = Vec::new();
            let leaves = Arc::new(Mutex::new(Vec::new()));

            for i in 0..threads {
                let seed = seed.clone();
                let files = files.clone();
                let out_dir = out_dir.clone();
                let leaves = leaves.clone();

                handles.push(thread::spawn(move || {
                    for j in (i..files.len()).step_by(threads) {
                        let file = &files[j];
                        let mut f = File::open(file).unwrap();
                        let mut buf = Vec::new();
                        f.read_to_end(&mut buf).unwrap();

                        let path = format!("leaf_{}", j);
                        let encrypted = encrypt(&seed[..], &path, &buf);

                        let node_path = out_dir.join("nodes").join(format!("{}.bin", path));
                        fs::write(node_path, encrypted).unwrap();

                        let mut leaves = leaves.lock().unwrap();
                        leaves.push(path);
                    }
                }));
            }

            for h in handles {
                h.join().unwrap();
            }

            let mut leaves = Arc::try_unwrap(leaves).unwrap().into_inner().unwrap();
            leaves.sort_by_key(|p| p.strip_prefix("leaf_").unwrap().parse::<usize>().unwrap());

            let mut level = leaves.clone();
            let mut depth = 0;

            while level.len() > 1 {
                let mut next = Vec::new();

                for (idx, pair) in level.chunks(2).enumerate() {
                    let left = &pair[0];
                    let right = if pair.len() > 1 { &pair[1] } else { left };

                    let data = format!("{}|{}", left, right).into_bytes();
                    let path = format!("node_{}_{}", depth, idx);

                    let encrypted = encrypt(&seed[..], &path, &data);
                    fs::write(out.join("nodes").join(format!("{}.bin", path)), encrypted).unwrap();

                    next.push(path);
                }

                level = next;
                depth += 1;
            }

            fs::write(out.join("root.commit"), level[0].as_bytes()).unwrap();
            println!("Vault created successfully with {} leaves", leaves.len());
        }

        Commands::Decode {
            vault,
            seed,
            out_dir,
            threads: _,
        } => {
            let seed_bytes = hex::decode(seed).unwrap();
            fs::create_dir_all(&out_dir).unwrap();

            let root = fs::read_to_string(vault.join("root.commit")).unwrap();
            let root = root.trim();

            println!("Extracting files from vault...");
            extract_node(&vault, &seed_bytes, root, &out_dir);
            println!("Files extracted to: {}", out_dir.display());
        }
    }
}
