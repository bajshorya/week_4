use crate::crypto::aes::encrypt;
use rand::RngCore;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

pub fn run(files: Vec<PathBuf>, out: PathBuf, threads: usize) {
    fs::create_dir_all(out.join("nodes")).unwrap();

    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    println!("Seed (save this!): {}", hex::encode(seed));

    let seed = Arc::new(seed);
    let files = Arc::new(files);
    let out_dir = Arc::new(out.clone());
    let leaves = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();

    for i in 0..threads {
        let seed = seed.clone();
        let files = files.clone();
        let out_dir = out_dir.clone();
        let leaves = leaves.clone();

        handles.push(thread::spawn(move || {
            for j in (i..files.len()).step_by(threads) {
                let mut buf = Vec::new();
                File::open(&files[j])
                    .unwrap()
                    .read_to_end(&mut buf)
                    .unwrap();

                let path = format!("leaf_{}", j);
                let encrypted = encrypt(&seed[..], &path, &buf);

                fs::write(
                    out_dir.join("nodes").join(format!("{}.bin", path)),
                    encrypted,
                )
                .unwrap();

                leaves.lock().unwrap().push(path);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    build_tree(out, seed, leaves);
}

fn build_tree(out: PathBuf, seed: Arc<[u8; 32]>, leaves: Arc<Mutex<Vec<String>>>) {
    let mut leaves = Arc::try_unwrap(leaves).unwrap().into_inner().unwrap();
    leaves.sort();

    let mut level = leaves.clone();
    let mut depth = 0;

    while level.len() > 1 {
        let mut next = Vec::new();

        for (i, pair) in level.chunks(2).enumerate() {
            let left = &pair[0];
            let right = pair.get(1).unwrap_or(left);
            let data = format!("{}|{}", left, right).into_bytes();
            let path = format!("node_{}_{}", depth, i);

            let enc = encrypt(&seed[..], &path, &data);
            fs::write(out.join("nodes").join(format!("{}.bin", path)), enc).unwrap();
            next.push(path);
        }

        level = next;
        depth += 1;
    }

    fs::write(out.join("root.commit"), level[0].as_bytes()).unwrap();
}
