use std::{env, fs, path::PathBuf, process::ExitCode, str};
use stolon::hash::HasherCracker;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("\n\nusage: stolon_hash 'path/to/wordlist' 'password-hash'");
        // 'password123' => 'ef92b778bafe771e89245b89ecbc08a44a4e166c06659911881f383d4473e94f'
        println!("example: stolon_hash 'rockyou.txt' 'ef773d4473e94f...' Sha256\n\n");
        return ExitCode::FAILURE;
    }

    let mut filepath = PathBuf::new();
    filepath.push::<String>(args[1].parse().unwrap());
    let hashed: Vec<u8> = hex::decode(args[2].parse::<String>().unwrap().as_bytes()).unwrap();

    let err = format!("unable to open file: {:?}", &filepath);
    let wordlist = fs::read(&filepath).expect(&err[..]);

    // TODO: figure out the hash algorithm based on magic numbers
    let crack_hash = args[3].as_str().crack();

    if let Some(result) = crack_hash(&wordlist[..], &hashed[..]) {
        println!(
            "\n\ncracked password is: \t{:?}\n\n",
            str::from_utf8(result).unwrap()
        );
    } else {
        println!("no password was found for the hash: {:?}", &hashed);
    }

    ExitCode::SUCCESS
}
