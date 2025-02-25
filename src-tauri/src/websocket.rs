use crate::configs::app_list::AppList;
use crate::configs::ConfigFile;
use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error as WsError, Message, Result as WsResult},
};

async fn handle_connection(stream: TcpStream) -> WsResult<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            let response = handle_extension_message(msg).await;
            ws_sender.send(response).await?;
        } else if msg.is_close() {
            break;
        }
    }
    Ok(())
}

async fn handle_extension_message(msg: Message) -> Message {
    if let Message::Text(text) = &msg {
        let mut app_list = match AppList::read().await {
            Ok(list) => list,
            Err(_) => return Message::Text("Error: Failed to read app list".into()),
        };

        if app_list.has_link(text) {
            return Message::Text("Link already exists".into());
        }

        app_list.add_link(text.to_string());
        if let Err(_) = app_list.save().await {
            return Message::Text("Error: Failed to save link".into());
        }
        return Message::Text("Link added successfully".into());
    }
    Message::Text("Invalid message format".into())
}

pub async fn start_websocket_server() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
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
