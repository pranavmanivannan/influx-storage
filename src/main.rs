// main.rs
mod channel_messenger;
mod data;
mod data_ingestor;
use channel_messenger::ChannelMessenger;
use chrono::{DateTime, TimeZone, Utc};
use data::*;
use data_ingestor::DataIngestor;
use std::{sync::mpsc, thread, time::Duration};

#[tokio::main]
async fn main() {
    let input: DateTime<Utc> = Utc::now();
    input.to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
    let format = "%Y-%m-%d %H:%M:%S%.f %Z";

    match Utc.datetime_from_str(&input.to_string(), format) {
        Ok(datetime) => {
            let output = datetime.to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
            println!("Formatted date-time: {}", output);
        }
        Err(e) => println!("Error parsing date-time: {}", e),
    }

    let (tx, rx) = mpsc::channel();
    let mut channel_messenger = ChannelMessenger::new(tx);
    let mut data_ingestor = DataIngestor::new(rx, 100);

    let sender_handle = thread::spawn(move || loop {
        let data = DataEnum::MBP(MarketIncremental {
            asks: vec![(100.0, 1.0), (101.0, 2.0), (102.0, 3.0)],
            bids: vec![(98.0, 5.0), (97.0, 4.0), (96.0, 2.0)],
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
