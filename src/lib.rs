use sha1::{Digest, Sha1};

/// Usage: stolon-hash::crack_sha1(wordlist, password_hash)

#[allow(dead_code, unused)]
fn crack_sha1(wordlist: &Vec<&str>, hashed: &String) -> Option<String> {
    let cracked = wordlist
        .into_iter()
        .filter_map(|&w| {
            if hashed == &hex::encode(Sha1::digest(w.as_bytes())) {
                Some(w.to_string())
            } else {
                None
            }
        })
        .collect::<String>();

    if cracked.len() > 0 {
        Some(cracked)
    } else {
        None
    }
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
            crack_sha1(&vec!["dog", "frog", "frog", "bat"], &hashed),
            Some(password),
        )
    }
}
