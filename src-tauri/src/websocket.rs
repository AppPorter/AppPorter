use crate::configs::app_list::App;
use crate::configs::ConfigFile;
use crate::operations::download_file;
use crate::CHANNEL;
use crate::{configs::app_list::AppList, SubCommands};
use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error as WsError, Message, Result as WsResult},
};

// Handles WebSocket connection and processes incoming messages in a loop
async fn handle_connection(stream: TcpStream) -> WsResult<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_close() {
            break;
        }

        if msg.is_text() || msg.is_binary() {
            let response = handle_extension_message(msg).await;
            ws_sender.send(response).await?;
        }
    }
    Ok(())
}

// Processes incoming browser extension messages and manages app list updates
async fn handle_extension_message(msg: Message) -> Message {
    if let Message::Text(text) = &msg {
        let mut app_list = match AppList::read().await {
            Ok(list) => list,
            Err(_) => return Message::Text("Error: Failed to read app list".into()),
        };

        if app_list.has_link(text) {
            return Message::Text("Link already exists".into());
        }

        let timestamp = chrono::Utc::now().timestamp();
        let new_app = App {
            timestamp,
            installed: false,
            url: text.to_string(),
            ..Default::default()
        };
        app_list.links.push(new_app);
        let result = app_list.save().await;

        let downloaded = download_file(text.to_string()).await.unwrap_or_default();
        println!("Downloaded file path: {}", downloaded);

        let sender = CHANNEL.0.clone();
        tokio::spawn(async move {
            sender
                .send(SubCommands::InstallWithTimestamp(downloaded, timestamp))
                .unwrap();
        });

        match result {
            Ok(_) => Message::Text("Link added successfully".into()),
            Err(_) => Message::Text("Error: Failed to save link".into()),
        }
    } else {
        Message::Text("Invalid message format".into())
    }
}

// Starts WebSocket server on port 7535 for browser extension communication
pub async fn start_websocket_server() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = "127.0.0.1:7535";
    let listener = TcpListener::bind(&addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                // Only log unexpected errors
                if !matches!(
                    e,
                    WsError::ConnectionClosed | WsError::Protocol(_) | WsError::Utf8
                ) {
                    eprintln!("Error processing connection: {}", e);
                }
            }
        });
    }

    Ok(())
}
