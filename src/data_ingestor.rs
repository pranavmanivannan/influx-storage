use crate::data::{DataEnum, DataPacket};
use chrono::{DateTime, TimeZone, Utc};
use dotenv::dotenv;
use reqwest::{self, Client};
use std::env;
use std::sync::mpsc;
use std::time::SystemTime;

const ORGANIZATION: &str = "devteam"; // replace with organization name
const ORG_ID: &str = ""; // replace w org id

/// A struct for setting a channel receiver endpoint and uploading the messages to data storage services.
pub struct DataIngestor {
    pub client: reqwest::Client,
    pub receiver_endpoint: mpsc::Receiver<DataPacket>,
    pub binance: Buffer,
    pub huobi: Buffer,
    pub buffer_capacity: usize,
}

/// A struct for making a buffer
pub struct Buffer {
    pub storage: Vec<String>, // buffer
    pub bucket: String,       // bucketname
}

/// An implementation of DataIngestor w constructor and receiving/uploading methods
impl DataIngestor {
    /// Basic constructor for DataIngestor that takes in a Receiver<DataPacket> endpoint and buf_capacity
    pub fn new(endpoint: mpsc::Receiver<DataPacket>, buf_capacity: usize) -> DataIngestor {
        let client = reqwest::Client::new();
        DataIngestor {
            client: client,
            receiver_endpoint: endpoint,
            binance: Buffer::new("Binance".to_string()),
            huobi: Buffer::new("Huobi".to_string()),
            buffer_capacity: buf_capacity,
        }
    }

    /// A custom receive method which will receive messages and push to the buffer
    pub async fn receive_data(&mut self) {
        loop {
            match self.receiver_endpoint.recv() {
                Ok(data) => self.filter_buffer(data).await,
                Err(e) => println!("Unable to receive data: {:?}", e),
            }
        }
    }

    /// A separate function that sorts datapackets and pushes it to buffer
    async fn filter_buffer(&mut self, data_packet: DataPacket) {
        let buffer: &mut Buffer = match data_packet.exchange.as_str() {
            "Binance" => &mut self.binance,
            "Huobi" => &mut self.huobi,
            _ => return,
        };

        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp = duration_since_epoch.as_nanos(); // u128

        let message = match data_packet.data {
            DataEnum::MBP(msg) => {
                let asks_json =
                    serde_json::to_string(&msg.asks).unwrap_or_else(|_| "[]".to_string());
                let bids_json =
                    serde_json::to_string(&msg.bids).unwrap_or_else(|_| "[]".to_string());

                format!(
                    "{}-{} asks={:?},bids={:?} {}",
                    data_packet.channel.as_str(),
                    data_packet.symbol_pair.as_str(),
                    asks_json,
                    bids_json,
                    timestamp
                )
            }
            DataEnum::RBA(msg) => {
                let asks_json =
                    serde_json::to_string(&msg.asks).unwrap_or_else(|_| "[]".to_string());
                let bids_json =
                    serde_json::to_string(&msg.bids).unwrap_or_else(|_| "[]".to_string());

                format!(
                    "{}-{} asks={:?},bids={:?} {}",
                    data_packet.channel.as_str(),
                    data_packet.symbol_pair.as_str(),
                    asks_json,
                    bids_json,
                    timestamp
                )
            }
        };

        buffer.storage.push(message);

        //push to influx if capacity is reached
        if buffer.storage.len() > self.buffer_capacity {
            match buffer.push_data(&self.client).await {
                Ok(response_body) => {
                    println!("Successful Push");
                    buffer.storage.clear();

                    //time for testing purpose
                    let input: DateTime<Utc> = Utc::now();
                    input.to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
                    let format = "%Y-%m-%d %H:%M:%S%.f %Z";
                    let mut output: String = "".to_owned();
                    match Utc.datetime_from_str(&input.to_string(), format) {
                        Ok(datetime) => {
                            output = datetime.to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
                            println!("Formatted date-time: {}", output);
                        }
                        Err(e) => println!("Error parsing date-time: {}", e),
                    }

                    let _ = Buffer::query_data(
                        &self.client,
                        "2023-01-01T00:00:00Z".to_owned(),
                        output,
                        "Binance".to_owned(),
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

    /// Queries an InfluxDB bucket to get timeseries data through an HTTP request.
    pub async fn query_data(
        client: &Client,
        startdate: String, // "2023-01-01T00:00:00Z";  RFC3339 format
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
        print!("{}", data);

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
