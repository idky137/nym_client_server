// nym_client_socks5.rs [bin]
// use:
//

use nym_sdk::mixnet::{
    MixnetClientBuilder, MixnetMessageSender, Recipient, ReconstructedMessage, Socks5, StoragePaths,
};
use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tokio::io::{self, AsyncWriteExt};
use tokio_socks::tcp::Socks5Stream;

#[tokio::main]
async fn main() {
    nym_bin_common::logging::setup_logging();

    // create the client and connect to mixnet
    let storage_paths = StoragePaths::new_from_dir(PathBuf::from("/tmp/nym_client")).unwrap();
    let mut client = MixnetClientBuilder::new_with_default_storage(storage_paths)
        .await
        .unwrap()
        .build()
        .unwrap()
        .connect_to_mixnet()
        .await
        .unwrap();

    // display client address
    let our_address = client.nym_address();
    println!("\nClient nym address: {our_address}");

    // create the socks5_client and connect to mixnet
    let storage_paths =
        StoragePaths::new_from_dir(PathBuf::from("/tmp/nym_client_socks5")).unwrap();
    let socks5_config = Socks5::new(client.nym_address().to_string());
    let socks5_client = MixnetClientBuilder::new_with_default_storage(storage_paths)
        .await
        .unwrap()
        .socks5_config(socks5_config)
        .build()
        .unwrap()
        .connect_to_mixnet_via_socks5()
        .await
        .unwrap();

    // display socks5_client address
    let socks5_client_address = socks5_client.nym_address();
    println!("Socks5 client nym address: {}", socks5_client_address);

    // Establish a TCP connection to the target address through the
    let proxy_addr = socks5_client.socks5_url();
    let proxy_addr_trimmed = &proxy_addr[10..];
    println!("proxy_addr: {}", proxy_addr);
    let target_addr = "127.0.0.1:8080".to_string();
    let mut stream = Socks5Stream::connect(target_addr.as_str(), proxy_addr_trimmed)
        .await
        .unwrap();

    // send message through the mixnet
    println!("sending message through mixnet");
    stream.write_all(b"Client message here!").await.unwrap();

    // wait for response
    println!("waiting on response");
    let mut message: Vec<ReconstructedMessage> = Vec::new();
    while let Some(new_message) = client.wait_for_messages().await {
        if new_message.is_empty() {
            continue;
        }
        message = new_message;
        break;
    }
    let mut parsed = String::new();
    if let Some(r) = message.first() {
        parsed = String::from_utf8_lossy(&r.message).into_owned();
    }
    println!("\nmessaged recieved: {parsed}");

    thread::sleep(Duration::from_secs(10));

    // disconnect
    client.disconnect().await;
    socks5_client.disconnect().await;
}
