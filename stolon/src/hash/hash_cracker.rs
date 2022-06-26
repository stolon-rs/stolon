use crate::hash::hash_identity::HashIdentity;
use rayon::prelude::*;
use sha2::{Digest, Sha256, Sha512};
use std::str;

/// Usage: stolon-hash::crack::<Sha256>(wordlist_bytes, hashed_password)

pub trait HasherCracker {
    fn cracker<'a>(&self) -> fn(&'a [u8], &'a [u8]) -> Option<&'a [u8]>;
}

impl HasherCracker for &str {
    fn cracker<'a>(&self) -> fn(&'a [u8], &'a [u8]) -> Option<&'a [u8]> {
        if self.eq(&"SHA-256") {
            crack_with_algo::<Sha256>
        } else if self.eq(&"SHA-512") {
            crack_with_algo::<Sha512>
        } else {
            |_, _| None
        }
    }
}

pub fn crack<'a>(wordlist: &'a [u8], hashed: &'a [u8]) -> Option<&'a [u8]> {
    if let Some(algoname) = hex::encode(hashed).as_str().algorithm_name() {
        algoname.cracker()(wordlist, &hashed)
    } else {
        None
    }
}

pub fn crack_with_algo<'a, D>(wordlist: &'a [u8], hashed: &'a [u8]) -> Option<&'a [u8]>
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

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Sha256, Sha512};

    #[test]
    fn crack_algo_not_found() {
        assert_eq!(
            crack(["dog", "cat"].join("\n").as_bytes(), "87987979".as_bytes()),
            None
        );
    }

    #[test]
    fn crack_algo_found() {
        let hashed = Sha256::digest("frog");
        assert_eq!(
            crack(["dog", "frog", "bat"].join("\n").as_bytes(), &hashed),
            Some("frog".as_bytes())
        );
    }

    #[test]
    fn crack_algo_sha_negative_result() {
        let hashed = Sha256::digest("frog");
        assert_eq!(
            crack_with_algo::<Sha256>(["dog", "cat", "bat"].join("\n").as_bytes(), &hashed),
            None
        );

        assert_eq!(
            crack_with_algo::<Sha512>(["dog", "frog", "bat"].join("\n").as_bytes(), &hashed),
            None
        )
    }

    #[test]
    fn crack_algo_sha_positive_result() {
        let hashed = Sha256::digest("frog");
        assert_eq!(
            crack_with_algo::<Sha256>(["dog", "frog", "bat"].join("\n").as_bytes(), &hashed),
            Some("frog".as_bytes())
        );
    }

    #[test]
    fn crack_algo_sha_with_different_digest() {
        let hashed = Sha512::digest("frog");
        assert_eq!(
            crack_with_algo::<Sha512>(["dog", "frog", "frog"].join("\n").as_bytes(), &hashed),
            Some("frog".as_bytes())
        );
    }
}
