use criterion::{criterion_group, criterion_main, Criterion};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

pub fn crack<'a, D>(wordlist: &'a [u8], hashed: &'a [u8]) -> Option<&'a [u8]>
where
    D: Digest,
{
    wordlist
        .split(|i| i.is_ascii_whitespace())
        .into_iter()
        .find_map(|w| {
            if hashed.eq(&D::digest(w)[..]) {
                Some(w)
            } else {
                None
            }
        })
}

pub fn crack_with_rayon<'a, D>(wordlist: &'a [u8], hashed: &'a [u8]) -> Option<&'a [u8]>
where
    D: Digest,
{
    wordlist
        .par_split(|i| i.is_ascii_whitespace())
        .into_par_iter()
        .find_map_first(|w| {
            if hashed.eq(&D::digest(w)[..]) {
                Some(w)
            } else {
                None
            }
        })
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut filepath = PathBuf::new();
    filepath.push::<&str>("..");
    filepath.push::<&str>("stolon");
    filepath.push::<&str>("assets");
    filepath.push::<&str>("rockyou.txt");
    let err = format!("unable to open file: {:?}", &filepath);
    let wordlist = fs::read(&filepath).expect(&err[..]);
    let hashed =
        hex::decode("ef92b778bafe771e89245b89ecbc08a44a4e166c06659911881f383d4473e94f".as_bytes())
            .unwrap();

    c.bench_function("crack", |b| {
        b.iter(|| crack::<Sha256>(&wordlist[..], &hashed[..]))
    });
}

fn criterion_benchmark_par(c: &mut Criterion) {
    let mut filepath = PathBuf::new();
    filepath.push::<&str>("..");
    filepath.push::<&str>("stolon");
    filepath.push::<&str>("assets");
    filepath.push::<&str>("rockyou.txt");
    let err = format!("unable to open file: {:?}", &filepath);
    let wordlist = fs::read(&filepath).expect(&err[..]);
    let hashed =
        hex::decode("ef92b778bafe771e89245b89ecbc08a44a4e166c06659911881f383d4473e94f".as_bytes())
            .unwrap();

    c.bench_function("crack_with_rayon", |b| {
        b.iter(|| crack_with_rayon::<Sha256>(&wordlist[..], &hashed[..]))
    });
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_par);
criterion_main!(benches);
