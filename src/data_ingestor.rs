use std::sync::mpsc;

use crate::data::{DataEnum, DataPacket};
use chrono::Utc;
use reqwest;
use serde_json::{json, Value};

/// A struct for setting a channel receiver endpoint and uploading the messages to data storage services.
pub struct DataIngestor {
    pub client: reqwest::Client,
    pub receiver_endpoint: mpsc::Receiver<DataPacket>,
    pub buffer_manager: BufferManager,
    pub buffer_capacity: usize,
}

/// A struct for keeping track of all buffers needed to store and upload data to data storage service.
pub struct BufferManager {
    pub binance_market: Buffer,
    pub binance_trade: Buffer,
    pub huobi_market: Buffer,
    pub huobi_trade: Buffer,
}

/// A struct for making a buffer. It hold the buffer itself defined as a Vector of json values alongside
/// a string for the table the buffer will push data to.
pub struct Buffer {
    pub storage: Vec<Value>, // there is definitely a better name for this than storage
    pub table: String, // table/database name to use so that when we query we can call on this field
}

/// An implementation of DataIngestor with a constructor alongside methods for receiving messages from a channel
/// and uploading a buffer to a data storage service.
impl DataIngestor {
    /// Basic constructor for DataIngestor that takes in a Receiver<DataPacket> endpoint, a Buffers struct to hold the
    /// buffers it will store messages in, and a buffer capacity.
    pub fn new(
        endpoint: mpsc::Receiver<DataPacket>,
        bufs: BufferManager,
        buf_capacity: usize,
    ) -> DataIngestor {
        let client = reqwest::Client::new();
        DataIngestor {
            client: client,
            receiver_endpoint: endpoint,
            buffer_manager: bufs,
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
            ("Binance", "Market") => &mut self.buffer_manager.binance_market,
            ("Binance", "Trade") => &mut self.buffer_manager.binance_trade,
            ("Huobi", "Market") => &mut self.buffer_manager.huobi_market,
            ("Huobi", "Trade") => &mut self.buffer_manager.huobi_trade,
            _ => return,
        };

        let message = match data.Data {
            DataEnum::BBABinanceBTCData(msg) => {
                json!({"bestask": msg.bestask, "askamt": msg.askamt})
            }
            DataEnum::BBABinanceETHData(msg) => {
                json!({"bestask": msg.bestask, "askamt": msg.askamt})
            }
            DataEnum::BBAHuobiBTCData(msg) => {
                json!({"bestask": msg.bestask, "askamt": msg.askamt, "bestbid": msg.bidamt, "bidamt": msg.bidamt})
            }
            DataEnum::BBAHuobiETHData(msg) => {
                json!({"bestask": msg.bestask, "askamt": msg.askamt, "bestbid": msg.bidamt, "bidamt": msg.bidamt})
            }
        };

        buffer.storage.push(message);

        if buffer.storage.len() > self.buffer_capacity {
            BufferManager::write_data(&buffer);
            buffer.storage.clear();
        }
    }

    /// Writes the data to AWS Timestream through custom API Gateway
    pub async fn write(&self) -> Result<String, reqwest::Error> {
        let data = vec![
            json!({"name": "temp_name_1", "value": "10"}),
            json!({"name": "temp_name_2", "value": "20"}),
        ];

        let data_string = serde_json::to_string(&data).unwrap();

        let lambda_event = json!({
            "body": data_string,
            "httpMethod": "POST",
            "headers": {
                "Content-Type": "application/json"
            }
        });

        let api_gateway_url =
            "https://rht5rhsdzg.execute-api.us-east-1.amazonaws.com/upload_timestream";

        let response = self
            .client
            .post(api_gateway_url)
            .header("Content-Type", "application/json")
            .body(lambda_event.to_string())
            .send()
            .await?;
        let body = response.text().await?;
        Ok(body)
    }
}

impl BufferManager {
    /// Writes a buffer's data to Influx
    pub async fn write_data(buffer: &Buffer) {
        let influxdb_url = "";
        let organization = "";
    }

    /// Queries Influx to get timeseries data
    pub async fn query_data() {
        let influxdb_url = "";
        let organization = "";
    }

    /// Queries to check if a bucket exists. If it does, return the string. Else, create the bucket then
    /// return the string.
    pub async fn get_bucket() {
        let influxdb_url = "";
        let organization = "";
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
