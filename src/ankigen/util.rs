use sha2::{Digest, Sha256};

const BASE91: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn base91_encode(data: &[String]) -> String {
    let hash_str = data
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("__");

    let mut hash = Sha256::new();
    hash.update(hash_str.as_bytes());

    let hash_bytes = hash.finalize()[..8].to_vec();
    let mut hash_int = 0;
    for byte in hash_bytes {
        hash_int = hash_int << 8;
        hash_int += byte as u64;
    }

    let mut encoded = String::new();

    while hash_int > 0 {
        let remainder = (hash_int % 91) as usize;
        hash_int /= 91;
        encoded.push(BASE91.chars().nth(remainder).unwrap());
    }

    encoded.chars().rev().collect::<String>()
}
