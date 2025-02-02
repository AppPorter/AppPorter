use crate::configs::app_list::AppList;
use crate::configs::ConfigFile;
use futures_util::{SinkExt, StreamExt};
use log::*;
use std::{error::Error, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error as WsError, Message, Result as WsResult},
};

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> WsResult<()> {
    println!("New connection: {}", peer);
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket handshake: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        println!("Received message: {}: {:?}", peer, msg);

        if msg.is_text() || msg.is_binary() {
            let response = handle_extension_message(msg).await;
            println!("Response {}: {:?}", peer, response);
            ws_sender.send(response).await?;
        } else if msg.is_close() {
            println!("Close frame: {}", peer);
            break;
        }
    }

    println!("Closed 1: {}", peer);
    Ok(())
}

async fn handle_extension_message(msg: Message) -> Message {
    match &msg {
        Message::Text(text) => {
            let mut app_list = AppList::read().await.unwrap_or_default();
            app_list.add_link(text.to_string());
            if let Err(e) = app_list.save().await {
                error!("Failed to save app list: {}", e);
            }
            println!("Added link to app list: {}", text);
        }
        _ => println!("Unknown: {:?}", msg),
    }
    msg
}

pub async fn start_websocket_server() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await?;
    println!("started: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        println!("TCP: {}", peer);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(peer, stream).await {
                match e {
                    WsError::ConnectionClosed | WsError::Protocol(_) | WsError::Utf8 => {
                        println!("Closed 2: {}: {:?}", peer, e);
                    }
                    err => {
                        println!("Error processing connection with {}: {}", peer, err);
                        error!("Error processing connection: {}", err);
                    }
                }
            }
        });
    }

    Ok(())
}
