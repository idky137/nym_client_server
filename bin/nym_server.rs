// nym_server.rs [bin]
// use:
//

use nym_sdk::mixnet::{
    AnonymousSenderTag, MixnetClientBuilder, MixnetMessageSender, ReconstructedMessage,
    StoragePaths,
};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    nym_bin_common::logging::setup_logging();

    // create the server and connect to mixnet
    let storage_paths = StoragePaths::new_from_dir(PathBuf::from("/tmp/nym_server")).unwrap();
    let mut server = MixnetClientBuilder::new_with_default_storage(storage_paths)
        .await
        .unwrap()
        .build()
        .unwrap()
        .connect_to_mixnet()
        .await
        .unwrap();

    // display server address
    let our_address = server.nym_address();
    println!("\nServer nym address: {our_address}");
    println!("Waiting for message\n");

    // wait for message
    let mut message: Vec<ReconstructedMessage> = Vec::new();
    while let Some(new_message) = server.wait_for_messages().await {
        if new_message.is_empty() {
            continue;
        }
        message = new_message;
        break;
    }
    let mut parsed = String::new();
    if let Some(r) = message.first() {
        //parsed = String::from_utf8(r.message.clone()).unwrap();
        parsed = String::from_utf8_lossy(&r.message).into_owned();
    }
    println!("\nmessaged recieved: {parsed}");

    // parse AnonymousSenderTag
    let return_recipient = AnonymousSenderTag::try_from_base58_string(
        message[0].sender_tag.unwrap().to_base58_string(),
    )
    .unwrap();

    // print message recieved
    println!(
        "\nReceived message: {} \nfrom sender with surb bucket {}",
        parsed, return_recipient
    );

    // reply with AnonymousSenderTag
    println!("Replying using SURBs");
    server
        .send_reply(return_recipient, "Server response here!")
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(10));

    // disconnect
    server.disconnect().await;
}
