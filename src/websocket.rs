use futures::future;
use futures::prelude::*;
use std::io::Error as IoError;

use async_std::{
    net::{TcpListener, TcpStream},
    task,
};
use async_tungstenite::accept_async;
use std::sync::mpsc::Sender;

async fn handle_connection(sender: Sender<String>, raw_stream: TcpStream) {
    let ws_stream = accept_async(raw_stream).await.unwrap();
    let (_, incoming) = ws_stream.split();

    let mut x = String::new();
    let broadcast_incoming = incoming
        .try_filter(|msg| future::ready(!msg.is_close()))
        .try_for_each(|msg| {
            x = msg.to_text().unwrap().to_string();
            future::ok(())
        });

    broadcast_incoming.await;
    println!("{}", &x);
    sender.send(x);
}

pub async fn run_server(port: u16, sender: Sender<String>) -> Result<(), IoError> {
    let addr = format!("127.0.0.1:{}", port).to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.unwrap();
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, _)) = listener.accept().await {
        task::spawn(handle_connection(sender.clone(), stream));
    }

    Ok(())
}