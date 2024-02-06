// nym_client.rs [bin]
// use:
//

use nym_sdk::mixnet::{MixnetClientBuilder, MixnetMessageSender, Recipient, StoragePaths};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    nym_bin_common::logging::setup_logging();

    //  nym client config path
    let config_dir = PathBuf::from("/tmp/nym_client");
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
    println!("\nClient nym address: {our_address}");

    // Get server address from command line arguments
    let args: Vec<String> = env::args().collect();
    let address = Recipient::try_from_base58_string(args[1].clone()).unwrap();
    let recipient_address: &Recipient = &address;

    println!("\nSending message to address: {}", recipient_address);

    // Send a message through the mixnet
    client
        .send_plain_message(*recipient_address, "Client message here!")
        .await
        .unwrap();

    // wait for response
    println!("Waiting for response (once you see it, ctrl-c to exit)\n");
    client
        .on_messages(|msg| println!("\nReceived: {}", String::from_utf8_lossy(&msg.message)))
        .await;
}
