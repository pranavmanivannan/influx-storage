use crate::data::{DataEnum, DataPacket};
use dotenv::dotenv;
use reqwest::{self, Client};
use std::env;
use std::sync::mpsc;
use std::time::SystemTime;

const ORGANIZATION: &str = "devteam"; //replace with organization name
const ORG_ID: &str = ""; //replace w org id

/// A struct for setting a channel receiver endpoint and uploading the messages to data storage services.
pub struct DataIngestor {
    pub client: reqwest::Client,
    pub receiver_endpoint: mpsc::Receiver<DataPacket>,
    pub binance_market: Buffer,
    pub binance_trade: Buffer,
    pub huobi_market: Buffer,
    pub huobi_trade: Buffer,
    pub buffer_capacity: usize,
}

/// A struct for making a buffer. It hold the buffer itself defined as a Vector of json values alongside
/// a string for the table the buffer will push data to.
pub struct Buffer {
    pub storage: Vec<String>, // there is definitely a better name for this than storage
    pub bucket: String,
}

/// An implementation of DataIngestor with a constructor alongside methods for receiving messages from a channel
/// and uploading a buffer to a data storage service.
impl DataIngestor {
    /// Basic constructor for DataIngestor that takes in a Receiver<DataPacket> endpoint, a Buffers struct to hold the
    /// buffers it will store messages in, and a buffer capacity.
    pub fn new(endpoint: mpsc::Receiver<DataPacket>, buf_capacity: usize) -> DataIngestor {
        let client = reqwest::Client::new();
        DataIngestor {
            client: client,
            receiver_endpoint: endpoint,
            binance_market: Buffer::new("bucket_test".to_string()),
            binance_trade: Buffer::new("bucket_test".to_string()),
            huobi_market: Buffer::new("bucket_test".to_string()),
            huobi_trade: Buffer::new("bucket_test".to_string()),
            buffer_capacity: buf_capacity,
        }
    }

    /// A custom receive method which will receive messages and push them to the buffer. If the buffer reaches capacity,
    /// then it will call upload_data() to push the buffer's messages to a data storage service.
    pub async fn receive_data(&mut self) {
        loop {
            match self.receiver_endpoint.recv() {
                Ok(data) => self.filter_buffer(data).await,
                Err(e) => println!("Unable to receive data: {:?}", e),
            }
        }
    }

    /// A separate function that sorts the datapackets and pushes it to the appropriate buffer. If the buffer is
    /// full after pushing, it will upload the data to a data storage service then clear the buffer.
    async fn filter_buffer(&mut self, data_packet: DataPacket) {
        let buffer: &mut Buffer = match (data_packet.exchange.as_str(), data_packet.channel.as_str()) {
            ("Binance", "Market") => &mut self.binance_market,
            ("Binance", "Trade") => &mut self.binance_trade,
            ("Huobi", "Market") => &mut self.huobi_market,
            ("Huobi", "Trade") => &mut self.huobi_trade,
            _ => return,
        };

        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp = duration_since_epoch.as_nanos(); // u128

        let message = match data_packet.data {
            DataEnum::MBP(msg) => {
                format!(
                    "MBP, bestask={} askamt={} bestbid={} bidamt={} {}",
                    msg.bestask, msg.askamount, msg.bestbid, msg.bidamount, timestamp
                )
            },
            DataEnum::RBA(msg) => {
                format!(
                    "RBA, asks={:?} bids={:?} {}",
                    msg.asks, msg.bids, timestamp
                )
            },
        };

        buffer.storage.push(message);

        if buffer.storage.len() > self.buffer_capacity {
            match buffer.push_data(&self.client).await {
                Ok(response_body) => {
                    println!("Successful Push");
                    buffer.storage.clear();
                    let _ = Buffer::query_data(
                        &self.client,
                        "2023-01-01T00:00:00Z".to_owned(),
                        "2023-01-02T00:00:00Z".to_owned(),
                        "bucket_test".to_owned(),
                    )
                    .await;
                }
                Err(err) => {
                    eprintln!("Request failed: {:?}", err);
                }
            }
        }
    }
}

/// An implementation of the Buffer struct which allows Buffers
impl Buffer {
    fn new(bucket_name: String) -> Buffer {
        Buffer {
            storage: vec![],
            bucket: bucket_name,
        }
    }

    // let start_time = "2023-01-01T00:00:00Z";  RFC3339 format
    // let end_time = "2023-01-02T00:00:00Z";

    /// Queries an InfluxDB bucket to get timeseries data through an HTTP request.
    pub async fn query_data(
        client: &Client,
        startdate: String,
        enddate: String,
        bucket_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let organization = ORGANIZATION;
        let bucket_name = &bucket_name;

        let flux_query = "from(bucket: \"".to_owned()
            + bucket_name
            + "\")\n |> range(start: "
            + &startdate
            + ", stop: "
            + &enddate
            + ")";
        let api_token = env::var("API_TOKEN").expect("API_TOKEN must be set");

        let url = format!(
            "https://us-east-1-1.aws.cloud2.influxdata.com/api/v2/query?org={}",
            organization
        );

        let response = client
            .post(url)
            .header("Authorization", format!("Token {}", api_token))
            .header("Accept", "application/csv")
            .header("Content-type", "application/vnd.flux")
            .body(flux_query)
            .send()
            .await;

        //error handling can delete later
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let data = res.text().await?;
                    println!("Queried Data:\n{}", data);
                } else {
                    // handle non-successful status codes
                    eprintln!("Error: HTTP {}", res.status());
                    let error_text = res.text().await?;
                    if !error_text.is_empty() {
                        eprintln!("Error details: {}", error_text);
                    }
                }
                Ok(())
            }
            Err(error) => {
                // handle network or reqwest-specific errors
                Err(Box::new(error))
            }
        }
    }

    /// Pushes the data in a buffer to an InfluxDB bucket.
    pub async fn push_data(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let organization = ORGANIZATION;
        let data = self.storage.join("\n");
        let bucket_name = &self.bucket;

        let url = format!(
            "https://us-east-1-1.aws.cloud2.influxdata.com/api/v2/write?org={}&bucket={}",
            organization, bucket_name,
        );

        let api_token = env::var("API_TOKEN").expect("API_TOKEN must be set");

        let response = client
            .post(url)
            .header("Authorization", format!("Token {}", api_token))
            .header("Content-Type", "text/plain; charset=utf-8")
            .header("Accept", "application/json")
            .body(data)
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let data = res.text().await?;

                    println!("Pushed Data:\n{}", data);
                } else {
                    // Handle non-successful status codes
                    eprintln!("Error: HTTP {}", res.status());
                    let error_text = res.text().await?;
                    if !error_text.is_empty() {
                        eprintln!("Error details: {}", error_text);
                    }
                }
                Ok(())
            }
            Err(error) => {
                // Handle network or reqwest-specific errors
                Err(Box::new(error))
            }
        }
    }
}
