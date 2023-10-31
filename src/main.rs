mod awsuploader;
mod channelmessenger;
mod data;

use hyper::{self, Client};
use std::{sync::mpsc, thread, time::Duration};
use tokio::time::sleep;

use awsuploader::AWSUploader;
use channelmessenger::ChannelMessenger;
use data::MarketData;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let (tx, rx) = mpsc::channel();

    let sender = ChannelMessenger::new(tx);

    let mut receiver = AWSUploader::new(
        rx,
        vec![],
        100,
        "sampleDB".to_string(),
        "sampleTB".to_string(),
        client,
    );

    let sender_handle = thread::spawn(move || loop {
        let data = MarketData {
            id: 1,
            ts: 2,
            tick: 3,
        };
        sender.send_data(&data);
        thread::sleep(Duration::from_millis(100));
    });

    // Spawn a new thread
    let receiver_handle = thread::spawn(move || {
        // Create a Tokio runtime within the thread
        let rt = tokio::runtime::Builder::new_current_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("Unable to create Tokio runtime");

        // Run the asynchronous receiver task
        rt.block_on(async {
            loop {
                receiver.receive_data().await;
                // Add a sleep or other logic as needed
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    });
    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();
}
