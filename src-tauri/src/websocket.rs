use crate::configs::app_list::AppList;
use crate::configs::ConfigFile;
use crate::{configs::app_list::App, operations::install_with_link};
use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error as WsError, Message},
};

// Handles WebSocket connection and processes incoming messages in a loop
async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_close() {
            break;
        }

        if msg.is_text() || msg.is_binary() {
            let response = handle_extension_message(msg).await?;
            ws_sender.send(response).await?;
        }
    }
    Ok(())
}

// Processes incoming browser extension messages and manages app list updates
async fn handle_extension_message(msg: Message) -> Result<Message, Box<dyn Error + Send + Sync>> {
    if let Message::Text(text) = &msg {
        let mut app_list = match AppList::read().await {
            Ok(list) => list,
            Err(_) => return Ok(Message::Text("Error: Failed to read app list".into())),
        };

        if app_list.has_link(text) {
            return Ok(Message::Text("Link already exists".into()));
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

        install_with_link(text.to_string(), timestamp).await?;

        match result {
            Ok(_) => Ok(Message::Text("Link added successfully".into())),
            Err(_) => Ok(Message::Text("Error: Failed to save link".into())),
        }
    } else {
        Ok(Message::Text("Invalid message format".into()))
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
