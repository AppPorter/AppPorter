use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type SessionStore = Arc<Mutex<HashMap<String, [u8; 32]>>>;

lazy_static::lazy_static! {
    pub static ref SESSIONS: SessionStore = Arc::new(Mutex::new(HashMap::new()));
}

pub fn generate_session_id() -> String {
    let mut bytes = [0u8; 16];
    let mut rng = rand::rng();
    rng.fill(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
}

pub fn generate_session_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    let mut rng = rand::rng();
    rng.fill(&mut key);
    key
}

pub fn encrypt_data_with_key(data: &str, key: &[u8; 32]) -> Result<(String, String)> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let mut nonce_bytes = [0u8; 12];
    let mut rng = rand::rng();
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| anyhow!("Encryption failed: {}", e))?;

    let encrypted_b64 = general_purpose::STANDARD.encode(&ciphertext);
    let nonce_b64 = general_purpose::STANDARD.encode(nonce_bytes);

    Ok((encrypted_b64, nonce_b64))
}

pub fn decrypt_data_with_key(
    encrypted_b64: &str,
    nonce_b64: &str,
    key: &[u8; 32],
) -> Result<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let ciphertext = general_purpose::STANDARD.decode(encrypted_b64)?;
    let nonce_bytes = general_purpose::STANDARD.decode(nonce_b64)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| anyhow!("Decryption failed: {}", e))?;

    Ok(String::from_utf8(plaintext)?)
}
