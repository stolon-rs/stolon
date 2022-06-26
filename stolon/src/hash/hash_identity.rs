use phf::phf_map;

const ALGORITHMS: phf::Map<&'static str, &'static str> = phf_map! {"102020" => "ADLER-32",
"102040" => "CRC-32",
"102060" => "CRC-32B",
"101020" => "CRC-16",
"101040" => "CRC-16-CCITT",
"104020" => "DES(Unix)",
"101060" => "FCS-16",
"103040" => "GHash-32-3",
"103020" => "GHash-32-5",
"115060" => "GOST R 34.11-94",
"109100" => "Haval-160",
"109200" => "Haval-160(HMAC)",
"110040" => "Haval-192",
"110080" => "Haval-192(HMAC)",
"114040" => "Haval-224",
"114080" => "Haval-224(HMAC)",
"115040" => "Haval-256",
"115140" => "Haval-256(HMAC)",
"107080" => "Lineage II C4",
"106025" => "Domain Cached Credentials - MD4(MD4(($pass)).(strtolower($username)))",
"102080" => "XOR-32",
"105060" => "MD5(Half)",
"105040" => "MD5(Middle)",
"105020" => "MySQL",
"107040" => "MD5(phpBB3)",
"107060" => "MD5(Unix)",
"107020" => "MD5(Wordpress)",
"108020" => "MD5(APR)",
"106160" => "Haval-128",
"106165" => "Haval-128(HMAC)",
"106060" => "MD2",
"106120" => "MD2(HMAC)",
"106040" => "MD4",
"106100" => "MD4(HMAC)",
"106020" => "MD5",
"106080" => "MD5(HMAC)",
"106140" => "MD5(HMAC(Wordpress))",
"106029" => "NTLM",
"106027" => "RAdmin v2.x",
"106180" => "RipeMD-128",
"106185" => "RipeMD-128(HMAC)",
"106200" => "SNEFRU-128",
"106205" => "SNEFRU-128(HMAC)",
"106220" => "Tiger-128",
"106225" => "Tiger-128(HMAC)",
"106240" => "md5($pass.$salt)",
"106260" => "md5($salt.'-'.md5($pass))",
"106280" => "md5($salt.$pass)",
"106300" => "md5($salt.$pass.$salt)",
"106320" => "md5($salt.$pass.$username)",
"106340" => "md5($salt.md5($pass))",
"106360" => "md5($salt.md5($pass).$salt)",
"106380" => "md5($salt.md5($pass.$salt))",
"106400" => "md5($salt.md5($salt.$pass))",
"106420" => "md5($salt.md5(md5($pass).$salt))",
"106440" => "md5($username.0.$pass)",
"106460" => "md5($username.LF.$pass)",
"106480" => "md5($username.md5($pass).$salt)",
"106500" => "md5(md5($pass))",
"106520" => "md5(md5($pass).$salt)",
"106540" => "md5(md5($pass).md5($salt))",
"106560" => "md5(md5($salt).$pass)",
"106580" => "md5(md5($salt).md5($pass))",
"106600" => "md5(md5($username.$pass).$salt)",
"106620" => "md5(md5(md5($pass)))",
"106640" => "md5(md5(md5(md5($pass))))",
"106660" => "md5(md5(md5(md5(md5($pass)))))",
"106680" => "md5(sha1($pass))",
"106700" => "md5(sha1(md5($pass)))",
"106720" => "md5(sha1(md5(sha1($pass))))",
"106740" => "md5(strtoupper(md5($pass)))",
"109040" => "MySQL5 - SHA-1(SHA-1($pass))",
"109060" => "MySQL 160bit - SHA-1(SHA-1($pass))",
"109180" => "RipeMD-160(HMAC)",
"109120" => "RipeMD-160",
"109020" => "SHA-1",
"109140" => "SHA-1(HMAC)",
"109220" => "SHA-1(MaNGOS)",
"109240" => "SHA-1(MaNGOS2)",
"109080" => "Tiger-160",
"109160" => "Tiger-160(HMAC)",
"109260" => "sha1($pass.$salt)",
"109280" => "sha1($salt.$pass)",
"109300" => "sha1($salt.md5($pass))",
"109320" => "sha1($salt.md5($pass).$salt)",
"109340" => "sha1($salt.sha1($pass))",
"109360" => "sha1($salt.sha1($salt.sha1($pass)))",
"109380" => "sha1($username.$pass)",
"109400" => "sha1($username.$pass.$salt)",
"1094202" => "sha1(md5($pass))",
"109440" => "sha1(md5($pass).$salt)",
"109460" => "sha1(md5(sha1($pass)))",
"109480" => "sha1(sha1($pass))",
"109500" => "sha1(sha1($pass).$salt)",
"109520" => "sha1(sha1($pass).substr($pass,0,3))",
"109540" => "sha1(sha1($salt.$pass))",
"109560" => "sha1(sha1(sha1($pass)))",
"109580" => "sha1(strtolower($username).$pass)",
"110020" => "Tiger-192",
"110060" => "Tiger-192(HMAC)",
"112020" => "md5($pass.$salt) - Joomla",
"113020" => "SHA-1(Django)",
"114020" => "SHA-224",
"114060" => "SHA-224(HMAC)",
"115080" => "RipeMD-256",
"115160" => "RipeMD-256(HMAC)",
"115100" => "SNEFRU-256",
"115180" => "SNEFRU-256(HMAC)",
"115200" => "SHA-256(md5($pass))",
"115220" => "SHA-256(sha1($pass))",
"115020" => "SHA-256",
"115120" => "SHA-256(HMAC)",
"116020" => "md5($pass.$salt) - Joomla",
"116040" => "SAM - (LM_hash:NT_hash)",
"117020" => "SHA-256(Django)",
"118020" => "RipeMD-320",
"118040" => "RipeMD-320(HMAC)",
"119020" => "SHA-384",
"119040" => "SHA-384(HMAC)",
"120020" => "SHA-256",
"121020" => "SHA-384(Django)",
"122020" => "SHA-512",
"122060" => "SHA-512(HMAC)",
"122040" => "Whirlpool",
"122080" => "Whirlpool(HMAC)",
};

pub trait HashIdentity {
    fn algorithm_name<'a>(&self) -> Option<&'a str>;
}

impl HashIdentity for &str {
    fn algorithm_name<'a>(&self) -> Option<&'a str> {
        if is_sha256(self) {
            Some(ALGORITHMS["115020"])
        } else if is_sha512(self) {
            Some(ALGORITHMS["122020"])
        } else {
            None
        }
    }
}

fn is_sha256<'a>(hash: &'a str) -> bool {
    let hs = "2c740d20dab7f14ec30510a11f8fd78b82bc3a711abe8a993acdb323e78e6d5e";
    hash.len() == hs.len()
        && !hash.chars().all(char::is_numeric)
        && !hash.chars().all(char::is_alphabetic)
        && hash.chars().all(char::is_alphanumeric)
}

fn is_sha512<'a>(hash: &'a str) -> bool {
    let hs = "ea8e6f0935b34e2e6573b89c0856c81b831ef2cadfdee9f44eb9aa0955155ba5e8dd97f85c73f030666846773c91404fb0e12fb38936c56f8cf38a33ac89a24e";

    hash.len() == hs.len()
        && !hash.chars().all(char::is_numeric)
        && !hash.chars().all(char::is_alphabetic)
        && hash.chars().all(char::is_alphanumeric)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_sha256() {
        let hs = "ef92b778bafe771e89245b89ecbc08a44a4e166c06659911881f383d4473e94f";
        assert_eq!(is_sha256(hs), true);
    }

    #[test]
    fn identify_sha512() {
        let hs = "ea8e6f0935b34e2e6573b89c0856c81b831ef2cadfdee9f44eb9aa0955155ba5e8dd97f85c73f030666846773c91404fb0e12fb38936c56f8cf38a33ac89a24e";
        assert_eq!(is_sha512(hs), true);
    }
}
