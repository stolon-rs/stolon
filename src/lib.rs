use rayon::prelude::*;
use sha2::Digest;

/// Usage: stolon-hash::crack_sha::<Sha1>(wordlist, password_hash)

pub fn crack_sha<'a, D>(wordlist: &Vec<String>, hashed: &String) -> Option<String>
where
    D: Digest,
{
    wordlist.into_par_iter().find_map_first(|w| {
        let mut hasher = D::new();
        hasher.update(w.as_bytes());
        if *hashed == hex::encode(hasher.finalize()) {
            Some(w.to_owned())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Sha256, Sha512};

    fn setup<D>() -> (String, String)
    where
        D: Digest,
    {
        let password = "frog".to_string();
        let mut hasher = D::new();
        hasher.update(password.as_bytes());
        let hashed: String = hex::encode(hasher.finalize());
        (password, hashed)
    }

    #[test]
    fn crack_sha_negative_result() {
        let (_, hashed) = setup::<Sha256>();
        assert_eq!(
            crack_sha::<Sha256>(
                &vec![
                    String::from("dog"),
                    String::from("cat"),
                    String::from("bat")
                ],
                &hashed
            ),
            None
        );
    }

    #[test]
    fn crack_sha_positive_result() {
        let (password, hashed) = setup::<Sha256>();
        assert_eq!(
            crack_sha::<Sha256>(
                &vec![
                    String::from("dog"),
                    String::from("frog"),
                    String::from("bat")
                ],
                &hashed
            ),
            Some(password)
        );
    }

    #[test]
    fn crack_sha_generic_param() {
        let (password, hashed) = setup::<Sha512>();
        assert_eq!(
            crack_sha::<Sha512>(
                &vec![
                    String::from("dog"),
                    String::from("frog"),
                    String::from("frog")
                ],
                &hashed
            ),
            Some(password)
        );
    }
}
