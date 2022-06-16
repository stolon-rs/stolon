use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use sha2::{Digest, Sha256};
use stolon_hash::crack_sha;

fn main() {
    let filepath = Path::new("assets/rockyou.txt");
    let file = File::open(filepath).expect("unable to open file");
    let reader = BufReader::new(&file);
    let wordlist: Vec<String> = reader
        .lines()
        .filter_map(|line: Result<String, _>| match line {
            Ok(l) => Some(l),
            _ => None,
        })
        .collect();

    let mut hasher = Sha256::new();
    hasher.update("password123");
    let hashed = hex::encode(hasher.finalize());
    let result = crack_sha::<Sha256>(&wordlist, &hashed).unwrap();
    println!("now cracking {:?}...", hashed);
    println!("{:?}: \t{:?}", hashed, result);
}
