// main.rs
mod channel_messenger;
mod data;
mod data_ingestor;
use channel_messenger::ChannelMessenger;
use chrono::Utc;
use data::{BestBidAskDataBTCBinance, DataEnum, DataPacket};
use data_ingestor::{Buffer, DataIngestor};
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::{sync::mpsc, thread, time::Duration};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let mut channel_messenger = ChannelMessenger::new(tx);

    //tests
    // binance_market
    //     .storage
    //     .push("BBABinanceBTCData,best_ask=130 askamr=20 1700088084660515072".to_string());
    // binance_market
    //     .storage
    //     .push("BBABinanceBTCData,best_ask=140 askamr=20 1700088084660515072".to_string());
    // binance_market
    //     .storage
    //     .push("BBABinanceBTCData,best_ask=150 askamr=20 1700088084660515072".to_string());
    // binance_market
    //     .storage
    //     .push("BBABinanceBTCData,best_ask=160 askamr=20 1700088084660515072".to_string());

    // let sample_client = reqwest::Client::new();
    // match binance_market.push_data(&sample_client).await {
    //     Ok(response_body) => {
    //         println!("Successful push");
    //     }
    //     Err(err) => {
    //         eprintln!("Request failed: {:?}", err);
    //     }
    // }

    // match binance_market.query_data(&sample_client).await {
    //     Ok(response_body) => {
    //         println!("Successful query");
    //     }
    //     Err(err) => {
    //         eprintln!("Request failed: {:?}", err);
    //     }
    // }

    let mut data_ingestor = DataIngestor::new(rx, 100);

    let sender_handle = thread::spawn(move || loop {
        let data = DataEnum::BBABinanceBTCData(BestBidAskDataBTCBinance {
            bestask: 20.0,
            askamt: 20.0,
        });

        let data = DataPacket {
            Data: data,
            Exchange: "Binance".to_string(),
            Channel: "Market".to_string(),
            timestamp: 170016057719975295,
        };
        channel_messenger.send_data(data);
        thread::sleep(Duration::from_millis(100));
    });

    let receiver_handle = thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("Unable to create Tokio runtime");
        rt.block_on(async {
            loop {
                data_ingestor.receive_data().await;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    });

    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();
}
