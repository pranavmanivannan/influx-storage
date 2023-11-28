mod channel_messenger;
mod data;
mod data_ingestor;

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    fn test_new_channel_messenger() {}

    fn test_new_aws_uploader() {}

    fn test_receive_data() {}

    fn test_send_data() {}

    fn test_upload_data_success() {}

    fn test_upload_data_unsuccessful() {}

    fn test_upload_empty() {}

    fn performance_testing() {}

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
