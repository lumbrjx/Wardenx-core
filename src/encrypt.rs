use df1t_cyph::{df1t_decrypt, df1t_encrypt};
use digest::Digest;
use sha2::Sha256;

#[derive(PartialEq)]
pub enum EncType {
    ENC,
    DEC,
}
pub fn encrypt_pass(pass: String, salt: String) -> String {
    let res = df1t_encrypt(pass, salt);
    match res {
        Ok(t) => t,
        Err(e) => panic!(),
    }
}
pub fn decrypt_pass(pass: String, salt: String) -> String {
    let res = df1t_decrypt(pass, salt);
    match res {
        Ok(t) => t,
        Err(e) => panic!(),
    }
}

pub fn hash_string(input: &str) -> String {
    // Create new SHA-256 hasher
    let mut hasher = Sha256::new();

    // Update the hasher with the input string bytes
    hasher.update(input.as_bytes());

    // Finalize the hashing process and obtain the hash result as bytes
    let hash_result = hasher.finalize();

    // Convert the hash result bytes to a hexadecimal string
    let hash_hex = hash_result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    hash_hex
}
