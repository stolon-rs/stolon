use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use sha2::{Digest, Sha256};
use stolon_hash::crack;

fn main() {
    // let mut filepath = env::current_dir().expect("unable to get current dir");
    let mut filepath = env::current_dir().unwrap();
    filepath.push("assets");
    filepath.push("shortlist.txt");
    let file = File::open(&filepath).expect(&format!("unable to open file {:?}", &filepath)[..]);
    let reader = BufReader::new(&file);
    println!("reading the wordlist at: {:?}...", filepath);

    let wordlist = reader
        .lines()
        .filter_map(|l| match l {
            Ok(word) => Some(word),
            _ => None,
        })
        .collect::<Vec<String>>();

    println!("finished processing the wordlist...");

    let mut hasher = Sha256::new();
    hasher.update("password123");
    let hashed = hex::encode(hasher.finalize());

    if let Some(result) = crack::<Sha256>(&wordlist, &hashed) {
        println!("cracked password is: \t{:?}", result);
    } else {
        println!("no password was found for the hash: {:?}", hashed);
    }
}
