// main.rs
mod channel_messenger;
mod data;
mod data_ingestor;
use channel_messenger::ChannelMessenger;
use data::*;
use data_ingestor::DataIngestor;
use std::{sync::mpsc, thread, time::Duration};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let mut channel_messenger = ChannelMessenger::new(tx);
    let mut data_ingestor = DataIngestor::new(rx, 100);

    let sender_handle = thread::spawn(move || loop {
        let data = DataEnum::MBP(MarketIncremental {
            bestask: 100.0,
            askamount: 100.0,
            bestbid: 100.0,
            bidamount: 100.0,
        });

        let data_packet = DataPacket {
            data: data,
            exchange: data::ExchangeEnum::Binance,
            symbol_pair: data::SymbolEnum::BTCUSD,
            channel: "Market".to_string(),
            timestamp: 170016057719975295,
        };
        channel_messenger.send_data(data_packet);
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
