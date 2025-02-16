use crate::configs::app_list::AppList;
use crate::configs::ConfigFile;
use futures_util::{SinkExt, StreamExt};
use std::{collections::HashSet, error::Error, net::SocketAddr, sync::Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error as WsError, Message, Result as WsResult},
};

// Use a HashSet to store processed messages
lazy_static::lazy_static! {
    static ref PROCESSED_MESSAGES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> WsResult<()> {
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            if let Message::Text(text) = &msg {
                let msg_text = text.to_string();
                let should_process = {
                    let mut processed = PROCESSED_MESSAGES.lock().unwrap();
                    if !processed.contains(&msg_text) {
                        processed.insert(msg_text.clone());
                        true
                    } else {
                        false
                    }
                };

                if should_process {
                    let response = handle_extension_message(msg).await;
                    ws_sender.send(response).await?;
                }
            }
        } else if msg.is_close() {
            break;
        }
    }

    println!("Connection closed: {}", peer);
    Ok(())
}

async fn handle_extension_message(msg: Message) -> Message {
    if let Message::Text(text) = &msg {
        let mut app_list = match AppList::read().await {
            Ok(list) => list,
            Err(_) => {
                println!("Failed to read app list");
                return Message::Text("Error: Failed to read app list".into());
            }
        };

        if app_list.has_link(text) {
            println!("Link already exists: {}", text);
            return Message::Text("Link already exists".into());
        }

        app_list.add_link(text.to_string());
        if let Err(e) = app_list.save().await {
            println!("Failed to save app list: {}", e);
            return Message::Text("Error: Failed to save link".into());
        }
        println!("Successfully added link: {}", text);
        return Message::Text("Link added successfully".into());
    }
    Message::Text("Invalid message format".into())
}

pub async fn start_websocket_server() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(peer, stream).await {
                match e {
                    WsError::ConnectionClosed | WsError::Protocol(_) | WsError::Utf8 => (),
                    err => println!("Error processing connection: {}", err),
                }
            }
        });
    }

    Ok(())
}
