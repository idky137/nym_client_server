// nym_server.rs [bin]
// use:
//

use nym_sdk::mixnet::{
    AnonymousSenderTag, MixnetClientBuilder, MixnetMessageSender, ReconstructedMessage,
    StoragePaths,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    nym_bin_common::logging::setup_logging();

    // nym client config path
    let config_dir = PathBuf::from("/tmp/nym_server");
    let storage_paths = StoragePaths::new_from_dir(&config_dir).unwrap();

    // create the client and connect to mixnet
    let client = MixnetClientBuilder::new_with_default_storage(storage_paths)
        .await
        .unwrap()
        .build()
        .unwrap();
    let mut client = client.connect_to_mixnet().await.unwrap();

    // display client address
    let our_address = client.nym_address();
    println!("\nServer nym address: {our_address}");
    println!("Waiting for message\n");

    // recieve message - discard the empty vec sent along with a potential SURB topup request
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
        parsed = String::from_utf8(r.message.clone()).unwrap();
    }
    // parse AnonymousSenderTag
    let return_recipient = AnonymousSenderTag::try_from_base58_string(
        message[0].sender_tag.unwrap().to_base58_string(),
    )
    .unwrap();

    println!(
        "\nReceived message: {} \nfrom sender with surb bucket {}",
        parsed, return_recipient
    );

    // reply with AnonymousSenderTag
    println!("Replying using SURBs");
    client
        .send_reply(return_recipient, "Server response here!")
        .await
        .unwrap();

    // wait for future messages (panics if closes here..?)
    println!("Waiting for message (once you see it, ctrl-c to exit)\n");
    client
        .on_messages(|msg| println!("\nReceived: {}", String::from_utf8_lossy(&msg.message)))
        .await;
}
