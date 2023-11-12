use aws_config::SdkConfig;
use aws_sdk_timestreamquery::types::DimensionValueType;
use aws_sdk_timestreamwrite::{
    types::{Dimension, Record},
    Client as AWSClient,
};
// use aws_sdk_timestreamwrite::model::Record::CommonAttributes;
use aws_types::region::Region;
use hyper::{Client, Request, Response, Uri};
use std::{str::FromStr, sync::mpsc};

use crate::data;
use crate::data::DataPacket;

/// A struct for setting a channel receiver endpoint and uploading the messages to AWS services.
pub struct AWSUploader<DataPacket> {
    pub receiver_endpoint: mpsc::Receiver<DataPacket>,
    pub binance_market_buffer: Vec<DataPacket>,
    pub binance_trade_buffer: Vec<DataPacket>,
    pub huobi_market_buffer: Vec<DataPacket>,
    pub huobi_trade_buffer: Vec<DataPacket>,
    pub buffer_capacity: usize,
    pub database_name: String,
    pub table_name: String,
    pub client: Client<hyper::client::HttpConnector>,
}

/// An implementation of AWSUploader with a constructor alongside methods for receiving messages from a channel
/// and uploading a buffer to AWS.
impl<T> AWSUploader<T>
where
    T: data::Data,
{
    /// Basic constructor for AWSUploader that takes in a Receiver<T> endpoint, a buffer to hold messages from the
    /// channel, a buffer capacity, an AWS Timestream database name, and an AWS Timestream table name.
    pub fn new(
        endpoint: mpsc::Receiver<DataPacket>,
        binance_market_buffer: Vec<DataPacket>,
        binance_trade_buffer: Vec<DataPacket>,
        huobi_market_buffer: Vec<DataPacket>,
        huobi_trade_buffer: Vec<DataPacket>,
        buf_capacity: usize,
        db_name: String,
        tb_name: String,
        cli: Client<hyper::client::HttpConnector>,
    ) -> AWSUploader<DataPacket> {
        AWSUploader {
            receiver_endpoint: endpoint,
            binance_market_buffer: binance_market_buffer,
            binance_trade_buffer: binance_trade_buffer,
            huobi_market_buffer: huobi_market_buffer,
            huobi_trade_buffer: huobi_trade_buffer,
            buffer_capacity: buf_capacity,
            database_name: db_name,
            table_name: tb_name,
            client: cli,
        }
    }

    /// A custom receive method which will receive messages and push them to the buffer. If the buffer reaches capacity,
    /// then it will call upload_data() to push the buffer's messages to AWS.
    pub async fn receive_data(&mut self) {
        loop {
            match self.receiver_endpoint.recv() {
                Ok(data) => self.filter_buffer(data).await,
                Err(e) => println!("Unable to receive data: {:?}", e),
            }
            println!("Working"); // for testing
        }
        }
    }


    // A separate function that sorts the datapackets and pushes it to the buffer
    async fn filter_buffer(&mut self, data: DataPacket) {
        let buffer = match (data.Exchange.as_str(), data.Channel.as_str()) {
            ("Binance", "Market") => &mut self.binance_market_buffer,
            ("Binance", "Trade") => &mut self.binance_trade_buffer,
            ("Huobi", "Market") => &mut self.huobi_market_buffer,
            ("Huobi", "Trade") => &mut self.huobi_trade_buffer,
            _ => return, 
        };
    
        if buffer.len() < self.buffer_capacity {
            buffer.push(data);
        } else {
            self.upload_data().await;
            println!("Uploaded data!");
            buffer.clear(); 
            buffer.push(data); 
        }
    }
    /// A method that will upload data to AWS. It contains checks to ensure that there is an existing Timestream
    /// database and table, and will create them if necessary. After uploading data, the buffer will be cleared so
    /// future messages can be added.
    async fn upload_data(&mut self) {
        println!("1");
        let region = Region::new("us-east-1");
        println!("1.5");
        let config = SdkConfig::builder().region(region).build();
        println!("2");
        // let config = aws_config::load_from_env().await;
        let client = AWSClient::new(&config);
        println!("3");

        let common_attributes = Record::builder()
            .measure_name("cpu_usage")
            .dimensions(Dimension::builder().name("host").value("host1").build())
            .build();

        let new_record = Record::builder()
            .measure_name("cpu_usage")
            .measure_value("13.5")
            .time(chrono::Utc::now().to_rfc3339())
            .dimensions(Dimension::builder().name("host").value("host1").build())
            .build();
        println!("4");

        let request = client
            .write_records()
            .database_name("sampleDB")
            .table_name("sampleTB")
            .common_attributes(common_attributes)
            .records(new_record)
            .send()
            .await
            .unwrap_or_else(|e| {
                println!("Error: {}", e);
                panic!("Error sending data to timestream");
            });
        println!("5");

        self.buffer.clear();
    }
}
