mod channel_messenger;
mod data;
mod data_ingestor;

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{channel, self};
    use std::time::Duration;
    use tokio::runtime::Runtime;
    use super::*;
    use reqwest::{Client, StatusCode};
    use std::{env, thread};
    use dotenv::dotenv;

    use crate::channel_messenger::ChannelMessenger;
    use crate::data::{self, *};
    use crate::data_ingestor::{DataIngestor, Buffer};

    #[tokio::test]
    async fn test_query_data_binance_success() {
        dotenv().ok();
        let client = Client::new();
        let startdate = "2023-01-01T00:00:00Z".to_string();
        let enddate = "2023-01-02T00:00:00Z".to_string();
        let bucket_name = "Binance".to_string();

        let result = Buffer::query_data(&client, startdate, enddate, bucket_name).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query_data_binance_failure() {
        dotenv().ok();
        let client = Client::new();
        let startdate = "2023-01-04T00:00:00Z".to_string();
        let enddate = "2023-01-02T00:00:00Z".to_string();
        let bucket_name = "Bad_bucket".to_string();

        let response = Buffer::query_data(&client, startdate, enddate, bucket_name).await;

        // fix this after changing query/push
        // assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_normal_data() {
        let (tx, rx) = mpsc::channel();
        let channel_messenger = ChannelMessenger::new(tx);
        let mut data_ingestor = DataIngestor::new(rx, 100);

        let sender_handle = thread::spawn(move || {
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
        });

        let receiver_handle = thread::spawn(move || {
            async {
                    data_ingestor.receive_data().await;
                    assert!(data_ingestor.binance.storage.len() > 0);
            };
        });
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_empty_data() {
        let (tx, rx) = mpsc::channel();
        let channel_messenger = ChannelMessenger::new(tx);
        let mut data_ingestor = DataIngestor::new(rx, 100);

        let sender_handle = thread::spawn(move || {
            let data = DataEnum::MBP(MarketIncremental {
                asks: vec![],
                bids: vec![],
            });

            let data_packet = DataPacket {
                data: data,
                exchange: data::ExchangeEnum::Binance,
                symbol_pair: data::SymbolEnum::BTCUSD,
                channel: "Market".to_string(),
                timestamp: 170016057719975295,
            };
            channel_messenger.send_data(data_packet);
        });

        let receiver_handle = thread::spawn(move || {
            async {
                    data_ingestor.receive_data().await;
                    assert!(data_ingestor.binance.storage.len() > 0);
            };
        });
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_min_timestamp() {
        let (tx, rx) = mpsc::channel();
        let channel_messenger = ChannelMessenger::new(tx);
        let mut data_ingestor = DataIngestor::new(rx, 100);

        let sender_handle = thread::spawn(move || {
            let data = DataEnum::MBP(MarketIncremental {
                asks: vec![(100.0, 1.0), (101.0, 2.0), (102.0, 3.0)],
                bids: vec![(98.0, 5.0), (97.0, 4.0), (96.0, 2.0)],
            });

            let data_packet = DataPacket {
                data: data,
                exchange: data::ExchangeEnum::Binance,
                symbol_pair: data::SymbolEnum::BTCUSD,
                channel: "Market".to_string(),
                timestamp: std::i64::MIN,
            };
            channel_messenger.send_data(data_packet);
        });

        let receiver_handle = thread::spawn(move || {
            async {
                    data_ingestor.receive_data().await;
                    assert!(data_ingestor.binance.storage.len() > 0);
            };
        });
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_max_timestamp() {
        let (tx, rx) = mpsc::channel();
        let channel_messenger = ChannelMessenger::new(tx);
        let mut data_ingestor = DataIngestor::new(rx, 100);

        let sender_handle = thread::spawn(move || {
            let data = DataEnum::MBP(MarketIncremental {
                asks: vec![(100.0, 1.0), (101.0, 2.0), (102.0, 3.0)],
                bids: vec![(98.0, 5.0), (97.0, 4.0), (96.0, 2.0)],
            });

            let data_packet = DataPacket {
                data: data,
                exchange: data::ExchangeEnum::Binance,
                symbol_pair: data::SymbolEnum::BTCUSD,
                channel: "Market".to_string(),
                timestamp: std::i64::MAX,
            };
            channel_messenger.send_data(data_packet);
        });

        let receiver_handle = thread::spawn(move || {
            async {
                    data_ingestor.receive_data().await;
                    assert!(data_ingestor.binance.storage.len() > 0);
            };
        });
    }
    


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
}
