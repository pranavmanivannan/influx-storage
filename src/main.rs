// main.rs
mod awsuploader;
mod channelmessenger;
mod data;
use awsuploader::{AWSUploader, Buffers};
use channelmessenger::ChannelMessenger;
use data::{DataPacket, MessageType1};
use std::{sync::mpsc, thread, time::Duration};
mod api_gateway;
use hyper::{self, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let (tx, rx) = mpsc::channel();
    let sender = ChannelMessenger::new(tx);

    let buffers = Buffers {
        binance_market: vec![],
        binance_trade: vec![],
        huobi_market: vec![],
        huobi_trade: vec![],
    };

    //currently gets from rest api
    let awsuploader = AWSUploader::new(rx, buffers, 100);

    match awsuploader.write().await {
        Ok(response_body) => {
            println!("Response Body:\n{}", response_body);
        }
        Err(err) => {
            eprintln!("Request failed: {:?}", err);
        }
    }

    // let sender_handle = thread::spawn(move || loop {
    //     let message = MessageType1 {
    //         data: "data".to_string(),
    //         best_ask: 2.0,
    //         ask_amt: 3.0,
    //     };
    //     let data = DataPacket {
    //         temp_best_ask: "2".to_string(),
    //         temp_ask_amt: "1".to_string(),
    //         data: data::DataEnum::M1(message),
    //         exchange: "Huobi".to_string(),
    //         channel: "Trade".to_string(),
    //     };
    //     sender.send_data(data);
    //     thread::sleep(Duration::from_millis(100));
    // });
    // let receiver_handle = thread::spawn(move || {
    //     let rt = tokio::runtime::Builder::new_current_thread()
    //         .worker_threads(1)
    //         .enable_all()
    //         .build()
    //         .expect("Unable to create Tokio runtime");
    //     rt.block_on(async {
    //         loop {
    //             receiver.receive_data().await;
    //             tokio::time::sleep(Duration::from_secs(1)).await;
    //         }
    //     });
    // });
    // sender_handle.join().unwrap();
    // receiver_handle.join().unwrap();
    // Buffers::upload_data().await;
}
