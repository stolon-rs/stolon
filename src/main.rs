use sha2::{Digest, Sha256};
use std::{env, fs, str};
use stolon_hash::crack;

fn main() {
    // let mut filepath = env::current_dir().expect("unable to get current dir");
    let mut filepath = env::current_dir().unwrap();
    filepath.push("assets");
    filepath.push("rockyou.txt");
    let wordlist = fs::read(filepath).unwrap();

    let mut hasher = Sha256::new();
    hasher.update("password123");
    let hashed = hasher.finalize();

    if let Some(result) = crack::<Sha256>(&wordlist[..], &hashed[..]) {
        println!(
            "cracked password is: \t{:?}",
            str::from_utf8(result).unwrap()
        );
    } else {
        println!("no password was found for the hash: {:?}", hashed);
    }
}
