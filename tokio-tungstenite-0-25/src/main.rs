use std::io::Error;

use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = "127.0.0.1:3000".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let config = WebSocketConfig::default().read_buffer_size(64 * 1024);
    let ws_stream = tokio_tungstenite::accept_async_with_config(stream, Some(config))
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}
