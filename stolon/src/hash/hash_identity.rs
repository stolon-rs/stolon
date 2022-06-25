use crate::hash::hash_cracker::crack;
use sha2::{Sha256, Sha512};

pub trait Hasher {
    fn identify<'a>(&self) -> fn(&'a [u8], &'a [u8]) -> Option<&'a [u8]>;
}

impl Hasher for &str {
    fn identify<'a>(&self) -> fn(&'a [u8], &'a [u8]) -> Option<&'a [u8]> {
        if self.to_lowercase().eq("sha256") {
            crack::<Sha256>
        } else {
            crack::<Sha512>
        }
    }
}
