use magic_crypt::{new_magic_crypt, MagicCryptTrait};

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
