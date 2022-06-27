# Stolon

A collection of libraries used for writing fast, asynchronous, parallel penetration testing tools written in Rust.

[![GitHub tag][tag-badge]][tag-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[tag-badge]: https://img.shields.io/github/v/tag/stolonio/stolon
[tag-url]: https://github.com/stolonio/stolon/tags
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/stolonio/stolon/blob/main/LICENSE
[actions-badge]: https://github.com/stolonio/stolon/workflows/CI/badge.svg
[actions-url]: https://github.com/stolonio/stolon/actions?query=workflow%3ACI+branch%3Adevelop


## Example

```rust,no-run 
use std::{env, fs, path::PathBuf, process::ExitCode, str};
use stolon::hash::crack;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("\n\nusage: stolon_hash 'path/to/wordlist' 'password-hash'");
        println!("example: stolon_hash 'rockyou.txt' 'ef773d4473e94f...' \n\n");
        return ExitCode::FAILURE;
    }

    let mut filepath = PathBuf::new();
    filepath.push::<String>(args[1].parse().unwrap());
    let hashed: Vec<u8> = hex::decode(args[2].parse::<String>().unwrap().as_bytes()).unwrap();

    let err = format!("unable to open file: {:?}", &filepath);
    let wordlist = fs::read(&filepath).expect(&err[..]);

    if let Some(result) = crack(&wordlist[..], &hashed[..]) {
        println!(
            "\n\ncracked password is: \t{:?}\n\n",
            str::from_utf8(result).unwrap()
        );
    } else {
        println!("no password was found for the hash: {:?}", &hashed);
    }

    ExitCode::SUCCESS
}
```

