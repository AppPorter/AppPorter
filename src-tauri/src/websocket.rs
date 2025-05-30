use crate::core::download_file;
use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use tauri::AppHandle;
use tauri::Emitter;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error as WsError, Message},
};

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
    if let Message::Text(url) = &msg {
        let timestamp = chrono::Utc::now().timestamp();
        let downloaded = download_file(url.to_string()).await.unwrap_or_default();
        app.emit("received", (downloaded, timestamp))?;

        Ok(Message::Text("Success".into()))
    } else {
        Ok(Message::Text("Invalid message format".into()))
    }
}
