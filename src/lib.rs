use sha1::{Digest, Sha1};

/// Usage: stolon-hash::crack_sha1(wordlist, password_hash)

#[allow(dead_code, unused)]

fn crack_sha1<'a>(wordlist: &Vec<&'a str>, hashed: &String) -> Option<&'a str> {
    wordlist.into_iter().find_map(|w| {
        let mut hasher = Sha1::new();
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
    use sha1::{Digest, Sha1};

    use super::*;

    fn setup() -> (String, String) {
        let password = "frog".to_string();
        let hashed: String = hex::encode(Sha1::digest(password.as_bytes()));
        (password, hashed)
    }

    #[test]
    fn crack_sha1_negative_result() {
        let (_, hashed) = setup();
        assert_eq!(crack_sha1(&vec!["dog", "cat", "bat"], &hashed), None);
    }

    #[test]
    fn crack_sha1_positive_result() {
        let (password, hashed) = setup();
        assert_eq!(
            crack_sha1(&vec!["dog", "frog", "frog", "bat"], &hashed).unwrap(),
            password,
        )
    }
}
