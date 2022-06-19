use rayon::prelude::*;
use sha2::Digest;

/// Usage: stolon-hash::crack::<Sha256>(wordlist_bytes, hashed_password)

pub fn crack<'a, D>(wordlist: &'a [u8], hashed: &'a [u8]) -> Option<&'a [u8]>
where
    D: Digest,
{
    wordlist
        .par_split(|i| i.is_ascii_whitespace())
        .into_par_iter()
        .find_map_first(|w| {
            let mut hasher = D::new();
            hasher.update(w);
            if *hashed == hasher.finalize()[..] {
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
    fn crack_sha_negative_result() {
        let mut hasher = Sha256::new();
        hasher.update("frog");
        let hashed = hasher.finalize();
        assert_eq!(
            crack::<Sha256>(["dog", "cat", "bat"].join("\n").as_bytes(), &hashed),
            None
        );

        assert_eq!(
            crack::<Sha512>(["dog", "frog", "bat"].join("\n").as_bytes(), &hashed),
            None
        )
    }

    #[test]
    fn crack_sha_positive_result() {
        let mut hasher = Sha256::new();
        hasher.update("frog");
        let hashed = hasher.finalize();
        assert_eq!(
            crack::<Sha256>(["dog", "frog", "bat"].join("\n").as_bytes(), &hashed),
            Some("frog".as_bytes())
        );
    }

    #[test]
    fn crack_sha_with_different_digest() {
        let mut hasher = Sha512::new();
        hasher.update("frog");
        let hashed = hasher.finalize();
        assert_eq!(
            crack::<Sha512>(["dog", "frog", "frog"].join("\n").as_bytes(), &hashed),
            Some("frog".as_bytes())
        );
    }
}
