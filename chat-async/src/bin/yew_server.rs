use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebSocketMessage {
    message_type: String,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: Arc<Mutex<HashSet<String>>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();
    let mut username = String::new();

    loop {
        tokio::select! {
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From client {addr}: {text}");

                            let parsed: Result<WebSocketMessage, _> = serde_json::from_str(text);

                            if let Ok(ws_msg) = parsed {
                                if ws_msg.message_type == "register" {
                                    if let Some(name) = ws_msg.data {
                                        username = name.clone();

                                        {
                                            let mut users_lock = users.lock().unwrap();
                                            users_lock.insert(name);
                                        }

                                        let user_list: Vec<String> = {
                                            let users_lock = users.lock().unwrap();
                                            users_lock.iter().cloned().collect()
                                        };

                                        let response = WebSocketMessage {
                                            message_type: "users".to_string(),
                                            data_array: Some(user_list),
                                            data: None,
                                        };

                                        bcast_tx.send(serde_json::to_string(&response)?)?;
                                    }
                                } else if ws_msg.message_type == "message" {
                                    let response = WebSocketMessage {
                                        message_type: "message".to_string(),
                                        data_array: None,
                                        data: ws_msg.data,
                                    };

                                    bcast_tx.send(serde_json::to_string(&response)?)?;
                                }
                            }
                        }
                    }
                    Some(Err(e)) => {
                        println!("Error from {addr}: {e}");
                        break;
                    }
                    None => {
                        println!("Client {addr} disconnected");
                        break;
                    }
                }
            }

            msg = bcast_rx.recv() => {
                match msg {
                    Ok(msg) => {
                        ws_stream.send(Message::text(msg)).await?;
                    }
                    Err(e) => {
                        println!("Broadcast error: {e}");
                        break;
                    }
                }
            }
        }
    }

    if !username.is_empty() {
        {
            let mut users_lock = users.lock().unwrap();
            users_lock.remove(&username);
        }

        let user_list: Vec<String> = {
            let users_lock = users.lock().unwrap();
            users_lock.iter().cloned().collect()
        };

        let response = WebSocketMessage {
            message_type: "users".to_string(),
            data_array: Some(user_list),
            data: None,
        };

        let _ = bcast_tx.send(serde_json::to_string(&response)?);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let users = Arc::new(Mutex::new(HashSet::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Rust WebSocket server for YewChat listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");

        let bcast_tx = bcast_tx.clone();
        let users = users.clone();

        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;
            handle_connection(addr, ws_stream, bcast_tx, users).await
        });
    }
}

