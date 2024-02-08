// nym_client.rs [bin]
// use:
//

use nym_sdk::mixnet::{
    MixnetClientBuilder, MixnetMessageSender, Recipient, ReconstructedMessage, StoragePaths,
};
use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

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

    // Get server address from command line arguments
    let args: Vec<String> = env::args().collect();
    let recipient_address: &Recipient =
        &Recipient::try_from_base58_string(args[1].clone()).unwrap();

    // Send a message through the mixnet
    println!("\nSending message: to address: {}", recipient_address);
    client
        .send_plain_message(*recipient_address, "Client message here!")
        .await
        .unwrap();

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
}
