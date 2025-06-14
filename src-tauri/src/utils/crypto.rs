use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

// Session storage for dynamic keys
pub type SessionStore = Arc<Mutex<HashMap<String, [u8; 32]>>>;

// Global session store
lazy_static::lazy_static! {
    pub static ref SESSIONS: SessionStore = Arc::new(Mutex::new(HashMap::new()));
}

// Generates a unique session ID
pub fn generate_session_id() -> String {
    let mut bytes = [0u8; 16];
    let mut rng = rand::rng();
    rng.fill(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
}

// Generates a random session key
pub fn generate_session_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    let mut rng = rand::rng();
    rng.fill(&mut key);
    key
}

// Encrypts data using AES-GCM with provided key
pub fn encrypt_data_with_key(
    data: &str,
    key: &[u8; 32],
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    // Generate random nonce (12 bytes for GCM)
    let mut nonce_bytes = [0u8; 12];
    let mut rng = rand::rng();
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the data
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Encode to base64 for transmission
    let encrypted_b64 = general_purpose::STANDARD.encode(&ciphertext);
    let nonce_b64 = general_purpose::STANDARD.encode(nonce_bytes);

    Ok((encrypted_b64, nonce_b64))
}

// Decrypts data using AES-GCM with provided key
pub fn decrypt_data_with_key(
    encrypted_b64: &str,
    nonce_b64: &str,
    key: &[u8; 32],
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    // Decode from base64
    let ciphertext = general_purpose::STANDARD.decode(encrypted_b64)?;
    let nonce_bytes = general_purpose::STANDARD.decode(nonce_b64)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Decrypt the data
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decryption failed: {}", e))?;

    Ok(String::from_utf8(plaintext)?)
}
