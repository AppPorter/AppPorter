use crate::utils::crypto::{
    SESSIONS, decrypt_data_with_key, encrypt_data_with_key, generate_session_id,
    generate_session_key,
};
use crate::utils::download_file;
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tauri::AppHandle;
use tauri::Emitter;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error as WsError, Message},
};

// Starts WebSocket server on port 7535 for browser extension communication
pub async fn start_websocket_server(app: AppHandle) -> Result<()> {
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
                        | Some(WsError::Utf8(_))
                ) {
                    eprintln!("Error processing connection: {}", e);
                }
            }
        });
    }

    Ok(())
}

// Handles WebSocket connection and processes incoming messages in a loop
async fn handle_connection(app: AppHandle, stream: TcpStream) -> Result<()> {
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
async fn handle_extension_message(app: AppHandle, msg: Message) -> Result<Message> {
    if let Message::Text(message_data) = &msg {
        // Parse the incoming JSON message
        let parsed: Value = serde_json::from_str(message_data)?;

        if parsed["type"] == "handshake" {
            // Generate session key and return it
            let session_id = generate_session_id();
            let session_key = generate_session_key();

            // Store session key
            {
                let mut sessions = SESSIONS
                    .lock()
                    .map_err(|e| anyhow!("Failed to lock sessions: {}", e))?;
                sessions.insert(session_id.clone(), session_key);
            }

            // Encode session key for transmission
            let session_key_b64 = general_purpose::STANDARD.encode(session_key);

            return Ok(Message::text(
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
            let session_id = parsed["session_id"]
                .as_str()
                .ok_or(anyhow!("Missing session ID"))?;
            let encrypted_data = parsed["data"]
                .as_str()
                .ok_or(anyhow!("Missing encrypted data"))?;
            let nonce_str = parsed["nonce"].as_str().ok_or(anyhow!("Missing nonce"))?;

            // Get session key
            let session_key = {
                let sessions = SESSIONS
                    .lock()
                    .map_err(|e| anyhow!("Failed to lock sessions: {}", e))?;
                sessions
                    .get(session_id)
                    .copied()
                    .ok_or(anyhow!("Invalid session ID"))?
            };

            // Decrypt the URL using session key
            if let Ok(decrypted_url) =
                decrypt_data_with_key(encrypted_data, nonce_str, &session_key)
            {
                let timestamp = chrono::Utc::now().timestamp();
                let downloaded = download_file(&decrypted_url).await?;
                app.emit("preview_url", (downloaded, timestamp, decrypted_url))?;

                // Return encrypted success response using session key
                let success_msg = "Success";
                let (encrypted_response, response_nonce) =
                    encrypt_data_with_key(success_msg, &session_key)?;

                return Ok(Message::text(
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
        let downloaded = download_file(&url).await?;
        app.emit("preview_url", (downloaded, timestamp, url))?;

        Ok(Message::text("Success"))
    } else {
        Ok(Message::text("Invalid message format"))
    }
}
