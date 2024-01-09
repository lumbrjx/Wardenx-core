use digest::Digest;
use sha2::Sha256;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
};
#[derive(PartialEq)]
pub enum EncType {
    ENC,
    DEC,
}
pub fn encrypt_pass(pass: String, enc_t: EncType) -> Result<Vec<u8>, aes_gcm::Error> {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    if enc_t == EncType::ENC {
        let ciphertext = cipher.encrypt(&nonce, pass.as_ref())?;
        println!("{:?}", ciphertext);
        return Ok(ciphertext);
    } else {
        let plaintext = cipher.decrypt(&nonce, pass.as_ref())?;
        println!("{:?}", plaintext);

        return Ok(plaintext);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_and_decrypt() {
        // Arrange
        let password = "secret_password".to_string();

        // Act
        let encrypted_result = encrypt_pass(password.clone(), EncType::ENC);
        assert!(encrypted_result.is_ok());
        let encrypted_password = encrypted_result.unwrap();
        if let Ok(s) = String::from_utf8(encrypted_password) {
            let decrypted_result = encrypt_pass(s, EncType::DEC);
            assert!(decrypted_result.is_ok());
            let decrypted_password = decrypted_result.unwrap();
            assert_eq!(password.as_bytes(), decrypted_password.as_slice());
        } else {
            println!("Invalid UTF-8 sequence");
        }

        // Assert
    }

    #[test]
    fn test_invalid_decryption() {
        // Arrange
        let invalid_password = vec![1, 2, 3]; // Invalid ciphertext
        if let Ok(s) = String::from_utf8(invalid_password) {
            let result = encrypt_pass(s, EncType::DEC);

            // Assert
            assert!(result.is_err());
        } else {
            println!("Invalid UTF-8 sequence");
        }
        // Act
    }
}
