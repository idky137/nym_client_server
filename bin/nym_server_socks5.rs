// nym_server_socks5.rs [bin]
// use:
//

use nym_sdk::mixnet::{
    AnonymousSenderTag, MixnetClientBuilder, MixnetMessageSender, ReconstructedMessage,
    StoragePaths,
};
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_socks::tcp::Socks5Listener;

#[tokio::main]
async fn main() {
    nym_bin_common::logging::setup_logging();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                // Read data into the buffer
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("Connection closed by {}", addr);
                        return; // connection was closed
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket: {:?}", e);
                        return;
                    }
                };

                // Print the received data
                let received_data = String::from_utf8_lossy(&buf[0..n]);
                println!("Received data from {}: {}", addr, received_data);
            }
        });
    }
}
