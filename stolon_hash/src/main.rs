use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf, process::ExitCode, str};
use stolon_hash::crack;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("\n\nusage: stolon_hash 'path/to/wordlist'\n\n");
        return ExitCode::FAILURE;
    }

    let mut filepath = PathBuf::new();
    filepath.push::<String>(args[1].parse().unwrap());

    let err = format!("unable to open file: {:?}", &filepath);
    let wordlist = fs::read(&filepath).expect(&err[..]);

    let mut hasher = Sha256::new();
    hasher.update("password123");
    let hashed = hasher.finalize();

    if let Some(result) = crack::<Sha256>(&wordlist[..], &hashed[..]) {
        println!(
            "\n\ncracked password is: \t{:?}\n\n",
            str::from_utf8(result).unwrap()
        );
    } else {
        println!("no password was found for the hash: {:?}", hashed);
    }

    ExitCode::SUCCESS
}
