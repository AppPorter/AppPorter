use crate::core::download_file;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use futures_util::{SinkExt, StreamExt};
use rand::RngCore;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::Emitter;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error as WsError, Message},
};

// Session storage for dynamic keys
type SessionStore = Arc<Mutex<HashMap<String, [u8; 32]>>>;

// Global session store
lazy_static::lazy_static! {
    static ref SESSIONS: SessionStore = Arc::new(Mutex::new(HashMap::new()));
}

// Starts WebSocket server on port 7535 for browser extension communication
pub async fn start_websocket_server(app: AppHandle) -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = "127.0.0.1:7535";
    let listener = TcpListener::bind(&addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        let app = app.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(app, stream).await {
                // Only log unexpected errors
                if !matches!(
                    e.downcast_ref::<WsError>(),
                    Some(WsError::ConnectionClosed)
                        | Some(WsError::Protocol(_))
                        | Some(WsError::Utf8)
                ) {
                    eprintln!("Error processing connection: {}", e);
                }
            }
        });
    }

    Ok(())
}

// Handles WebSocket connection and processes incoming messages in a loop
async fn handle_connection(
    app: AppHandle,
    stream: TcpStream,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_close() {
            break;
        }

        if msg.is_text() {
            let response = handle_extension_message(app.clone(), msg).await?;
            ws_sender.send(response).await?;
        }
    }
    Ok(())
}

// Processes incoming browser extension messages and manages app list updates
async fn handle_extension_message(
    app: AppHandle,
    msg: Message,
) -> Result<Message, Box<dyn Error + Send + Sync>> {
    if let Message::Text(message_data) = &msg {
        // Parse the incoming JSON message
        let parsed: Value = serde_json::from_str(message_data)?;

        if parsed["type"] == "handshake" {
            // Generate session key and return it
            let session_id = generate_session_id();
            let session_key = generate_session_key();

            // Store session key
            {
                let mut sessions = SESSIONS.lock().unwrap();
                sessions.insert(session_id.clone(), session_key);
            }

            // Encode session key for transmission
            let session_key_b64 = general_purpose::STANDARD.encode(&session_key);

            return Ok(Message::Text(
                json!({
                    "type": "handshake_response",
                    "status": "ready",
                    "session_id": session_id,
                    "session_key": session_key_b64
                })
                .to_string(),
            ));
        }

        if parsed["type"] == "encrypted_url" {
            let session_id = parsed["session_id"].as_str().ok_or("Missing session ID")?;
            let encrypted_data = parsed["data"].as_str().ok_or("Missing encrypted data")?;
            let nonce_str = parsed["nonce"].as_str().ok_or("Missing nonce")?;

            // Get session key
            let session_key = {
                let sessions = SESSIONS.lock().unwrap();
                sessions
                    .get(session_id)
                    .copied()
                    .ok_or("Invalid session ID")?
            };

            // Decrypt the URL using session key
            if let Ok(decrypted_url) =
                decrypt_data_with_key(encrypted_data, nonce_str, &session_key)
            {
                let timestamp = chrono::Utc::now().timestamp();
                let downloaded = download_file(&decrypted_url).await.unwrap_or_default();
                app.emit("preview_url", (downloaded, timestamp, decrypted_url))?;

                // Return encrypted success response using session key
                let success_msg = "Success";
                let (encrypted_response, response_nonce) =
                    encrypt_data_with_key(success_msg, &session_key)?;

                return Ok(Message::Text(
                    json!({
                        "type": "encrypted_response",
                        "data": encrypted_response,
                        "nonce": response_nonce,
                        "session_id": session_id
                    })
                    .to_string(),
                ));
            }
        }

        // Handle legacy unencrypted messages for backward compatibility
        let url = message_data.to_string();
        let timestamp = chrono::Utc::now().timestamp();
        let downloaded = download_file(&url).await.unwrap_or_default();
        app.emit("preview_url", (downloaded, timestamp, url))?;

        Ok(Message::Text("Success".into()))
    } else {
        Ok(Message::Text("Invalid message format".into()))
    }
}

// Generates a unique session ID
fn generate_session_id() -> String {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(&bytes)
}

// Generates a random session key
fn generate_session_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

// Encrypts data using AES-GCM with provided key
fn encrypt_data_with_key(
    data: &str,
    key: &[u8; 32],
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    // Generate random nonce (12 bytes for GCM)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the data
    let ciphertext = cipher.encrypt(nonce, data.as_bytes())?;

    // Encode to base64 for transmission
    let encrypted_b64 = general_purpose::STANDARD.encode(&ciphertext);
    let nonce_b64 = general_purpose::STANDARD.encode(&nonce_bytes);

    Ok((encrypted_b64, nonce_b64))
}

// Decrypts data using AES-GCM with provided key
fn decrypt_data_with_key(
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
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;

    Ok(String::from_utf8(plaintext)?)
}

// Legacy encryption functions for backward compatibility
fn encrypt_data(data: &str) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    // Use a default key for backward compatibility
    let default_key = b"AppPorter_WebSocket_Key_32bytes!";
    encrypt_data_with_key(data, default_key)
}

fn decrypt_data(
    encrypted_b64: &str,
    nonce_b64: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Use a default key for backward compatibility
    let default_key = b"AppPorter_WebSocket_Key_32bytes!";
    decrypt_data_with_key(encrypted_b64, nonce_b64, default_key)
}
