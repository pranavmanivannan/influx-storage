use crate::data::{DataEnum, DataPacket};
use chrono::Utc;
use reqwest::{self, Client};
use serde_json::{json, Value};
use std::time::{Duration, SystemTime};
use std::{hash::BuildHasher, sync::mpsc};

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
    pub table: String, // table/database name to use so that when we query we can call on this field
}

/// An implementation of DataIngestor with a constructor alongside methods for receiving messages from a channel
/// and uploading a buffer to a data storage service.
impl DataIngestor {
    /// Basic constructor for DataIngestor that takes in a Receiver<DataPacket> endpoint, a Buffers struct to hold the
    /// buffers it will store messages in, and a buffer capacity.
    pub fn new(
        endpoint: mpsc::Receiver<DataPacket>,
        binance_market: Buffer,
        binance_trade: Buffer,
        huobi_market: Buffer,
        huobi_trade: Buffer,
        buf_capacity: usize,
    ) -> DataIngestor {
        let client = reqwest::Client::new();
        DataIngestor {
            client: client,
            receiver_endpoint: endpoint,
            binance_market: binance_market,
            binance_trade: binance_trade,
            huobi_market: huobi_market,
            huobi_trade: huobi_trade,
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
            println!("Working"); // for testing
        }
    }

    /// A separate function that sorts the datapackets and pushes it to the appropriate buffer. If the buffer is
    /// full after pushing, it will upload the data to a data storage service then clear the buffer.
    async fn filter_buffer(&mut self, data: DataPacket) {
        let buffer: &mut Buffer = match (data.Exchange.as_str(), data.Channel.as_str()) {
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

        let message = match data.Data {
            DataEnum::BBABinanceBTCData(msg) => {
                format!(
                    "BBABinanceBTCData, best_ask={}, askamr={} {}",
                    msg.bestask, msg.askamt, timestamp
                )
                // json!({"bestask": msg.bestask, "askamt": msg.askamt})
            }
            DataEnum::BBABinanceETHData(msg) => {
                format!(
                    "BBABinanceETHData, best_ask={}, askamr={} {}",
                    msg.bestask, msg.askamt, timestamp
                )
                // json!({"bestask": msg.bestask, "askamt": msg.askamt})
            }
            DataEnum::BBAHuobiBTCData(msg) => {
                format!(
                    "BBAHuobiBTCData, best_ask={}, askamr={} {}",
                    msg.bestask, msg.askamt, timestamp
                )
                // json!({"bestask": msg.bestask, "askamt": msg.askamt, "bestbid": msg.bidamt, "bidamt": msg.bidamt})
            }
            DataEnum::BBAHuobiETHData(msg) => {
                format!(
                    "BBAHuobiETHData, best_ask={}, askamr={} {}",
                    msg.bestask, msg.askamt, timestamp
                )
                // json!({"bestask": msg.bestask, "askamt": msg.askamt, "bestbid": msg.bidamt, "bidamt": msg.bidamt})
            }
        };

        buffer.storage.push(message);

        if buffer.storage.len() > self.buffer_capacity {
            match buffer.push_data(&self.client).await {
                Ok(response_body) => {
                    println!("Successful Push");
                    buffer.storage.clear();
                }
                Err(err) => {
                    eprintln!("Request failed: {:?}", err);
                }
            }
        }
    }
}

impl Buffer {
    /// Queries Influx to get timeseries data through an HTTP request.
    pub async fn query_data(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let organization = "devteam"; // replace w org name
        let bucket_name = "bucket_test"; // replace w bucket name
        let flux_query = "from(bucket: \"".to_owned() + bucket_name + "\")\n |> range(start: -1h)"; // edit w custom q

        let api_token = ""; // replace with api token/env
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

    //pushes the data to influx db
    pub async fn push_data(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.storage.join("\n");
        let api_token = "";
        let client = reqwest::Client::new();
        let url = "https://us-east-1-1.aws.cloud2.influxdata.com/api/v2/write?org=devteam&bucket=bucket_test";

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

                    println!("Queried Data:\n{}", data);
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
    /// Checks if a bucket exists. If it does, return the string. Else, create the bucket then return the string.
    pub async fn get_bucket(client: &Client) -> String {
        let influxdb_url = "";
        let organization = "";
        // TO-DO
        let bucket_name = "";
        return bucket_name.to_string();
    }

    pub async fn create_bucket() -> Result<(), reqwest::Error> {
        // let influxdb_url = "http://localhost:8086";
        let influxdb_url = "https://us-east-1-1.aws.cloud2.influxdata.com";
        let organization = "pranavm";
        let token = "ACCESS";

        let bucket_name = "binance";
        let create_bucket_url = format!(
            "{}/api/v2/write?org={}&bucket={}&precision=ns",
            influxdb_url, organization, bucket_name
        );

        let req_body = r#"
            airSensors,sensor_id=TLM0201 temperature=73.97038159354763,humidity=35.23103248356096,co=0.48445310567793615 1630424257000000000
            airSensors,sensor_id=TLM0202 temperature=75.30007505999716,humidity=35.651929918691714,co=0.5141876544505826 1630424257000000000
        "#;

        let client = reqwest::Client::new();
        let response = client
            .post(&create_bucket_url)
            .header("Authorization", format!("Token {}", token))
            .header("Content-Type", "text/plain; charset=utf-8")
            .header("Accept", "application/json")
            .body(req_body)
            .send()
            .await?;

        println!("{:?}", response.status());

        Ok(())
    }
}
