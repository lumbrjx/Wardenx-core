use magic_crypt::{new_magic_crypt, MagicCryptTrait};
extern crate crypto_hash;
extern crate digest;
extern crate sha2;

use digest::Digest;
use sha2::Sha256;

pub fn encrypt_pass(pass: String) -> String {
    let mcrypt = new_magic_crypt!("magickey", 256); //Creates an instance of the magic crypt library/crate.
    let encrypted_string = mcrypt.encrypt_str_to_base64(pass); //Encrypts the string and saves it to the 'encrypted_string' variable.
    encrypted_string //Print the encrypted string to see if it worked.
}
pub fn decrypt_pass(pass: String) -> String {
    let mcrypt = new_magic_crypt!("magickey", 256); //Creates an instance of the magic crypt library/crate.
    let decrypted_string = mcrypt.decrypt_base64_to_string(&pass).unwrap(); //Decrypts the string so we can read it.
    decrypted_string //Print the human-readable, decrypted string.
}

pub fn hash_string(input: &str) -> String {
    // Create a new SHA-256 hasher
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
