// main.rs
mod channel_messenger;
mod data;
mod data_ingestor;
use channel_messenger::ChannelMessenger;
use chrono::Utc;
use data::DataPacket;
use data_ingestor::{Buffer, DataIngestor};
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::{sync::mpsc, thread, time::Duration};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let mut channel_messenger = ChannelMessenger::new(tx);

    let mut binance_market = Buffer {
        storage: vec![],
        table: "T".to_string(),
    };
    let binance_trade = Buffer {
        storage: vec![],
        table: "T".to_string(),
    };
    let huobi_market = Buffer {
        storage: vec![],
        table: "T".to_string(),
    };
    let huobi_trade = Buffer {
        storage: vec![],
        table: "T".to_string(),
    };
    //tests
    binance_market
        .storage
        .push("BBABinanceBTCData,best_ask=130 askamr=20 1700088084660515072".to_string());
    binance_market
        .storage
        .push("BBABinanceBTCData,best_ask=140 askamr=20 1700088084660515072".to_string());
    binance_market
        .storage
        .push("BBABinanceBTCData,best_ask=150 askamr=20 1700088084660515072".to_string());
    binance_market
        .storage
        .push("BBABinanceBTCData,best_ask=160 askamr=20 1700088084660515072".to_string());

    let sample_client = reqwest::Client::new();
    match binance_market.push_data(&sample_client).await {
        Ok(response_body) => {
            println!("Successful push");
        }
        Err(err) => {
            eprintln!("Request failed: {:?}", err);
        }
    }

    match binance_market.query_data(&sample_client).await {
        Ok(response_body) => {
            println!("Successful query");
        }
        Err(err) => {
            eprintln!("Request failed: {:?}", err);
        }
    }

    // let mut data_ingestor = DataIngestor::new(
    //     rx,
    //     binance_market,
    //     binance_trade,
    //     huobi_market,
    //     huobi_trade,
    //     100,
    // );

    // match awsuploader.write().await {
    //     Ok(response_body) => {
    //         println!("Response Body:\n{}", response_body);
    //     }
    //     Err(err) => {
    //         eprintln!("Request failed: {:?}", err);
    //     }
    // }

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
    //     channel_messenger.send_data(data);
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
    //             aws_uploader.receive_data().await;
    //             tokio::time::sleep(Duration::from_secs(1)).await;
    //         }
    //     });
    // });

    // sender_handle.join().unwrap();
    // receiver_handle.join().unwrap();
}
