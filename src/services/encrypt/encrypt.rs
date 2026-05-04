use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use base64::Engine;



type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt(plain_text: &str, key_b64: String, iv_b64: String) -> Vec<u8> {
    let key = base64::engine::general_purpose::STANDARD.decode(key_b64).unwrap();
    let iv = base64::engine::general_purpose::STANDARD.decode(iv_b64).unwrap();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plain_text.as_bytes());

    ciphertext
}

pub fn decrypt(input: &String, key_b64: String, iv_b64: String) -> Result<String, &'static str> {
    let decoded_input = base64::engine::general_purpose::STANDARD.decode(input).unwrap_or_default();
    let key = base64::engine::general_purpose::STANDARD.decode(key_b64).map_err(|_| "Invalid base64 for key")?;
    let iv = base64::engine::general_purpose::STANDARD.decode(iv_b64).map_err(|_| "Invalid base64 for IV")?;
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).map_err(|_| "Invalid key/iv length")?;
    let decrypted = cipher.decrypt_vec(&decoded_input).map_err(|_| "Decryption failed")?;
    
    String::from_utf8(decrypted).map_err(|_| "Invalid UTF-8")
}

