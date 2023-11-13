// use aws_config::SdkConfig;
// use aws_sdk_timestreamquery::types::DimensionValueType;
// use aws_sdk_timestreamwrite::{
//     types::{Dimension, Record},
//     Client as AWSClient,
// };
// // use aws_sdk_timestreamwrite::model::Record::CommonAttributes;
// use aws_types::region::Region;
// use hyper::{Client, Request, Response, Uri};
use std::{str::FromStr, sync::mpsc};

use crate::data::DataPacket;

// awsuploader.rs

use reqwest;

pub struct AWSUploader {
    pub client: reqwest::Client,
    pub receiver_endpoint: mpsc::Receiver<DataPacket>,
    pub buffers: Buffers,
    pub buffer_capacity: usize,
}

impl AWSUploader {
    pub fn new(
        endpoint: mpsc::Receiver<DataPacket>,
        bufs: Buffers,
        buf_capacity: usize,
    ) -> AWSUploader {
        let client = reqwest::Client::new();
        AWSUploader {
            client: client,
            receiver_endpoint: endpoint,
            buffers: bufs,
            buffer_capacity: buf_capacity,
        }
    }

    pub async fn get(&self) -> Result<String, reqwest::Error> {
        let url = "https://w6fxw1zkgg.execute-api.us-east-1.amazonaws.com/samplestage";
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;

        Ok(body)
    }
}

pub struct Buffers {
    pub binance_market: Vec<DataPacket>,
    pub binance_trade: Vec<DataPacket>,
    pub huobi_market: Vec<DataPacket>,
    pub huobi_trade: Vec<DataPacket>,
}

// /// A struct for setting a channel receiver endpoint and uploading the messages to AWS services.
// pub struct AWSUploader {
//     pub receiver_endpoint: mpsc::Receiver<DataPacket>,
//     pub buffers: Buffers,
//     pub buffer_capacity: usize,
//     pub client: Client<hyper::client::HttpConnector>,
// }

// /// An implementation of AWSUploader with a constructor alongside methods for receiving messages from a channel
// /// and uploading a buffer to AWS.
// impl AWSUploader {
//     /// Basic constructor for AWSUploader that takes in a Receiver<T> endpoint, a buffer to hold messages from the
//     /// channel, a buffer capacity, an AWS Timestream database name, and an AWS Timestream table name.
//     pub fn new(
//         endpoint: mpsc::Receiver<DataPacket>,
//         bufs: Buffers,
//         buf_capacity: usize,
//         cli: Client<hyper::client::HttpConnector>,
//     ) -> AWSUploader {
//         AWSUploader {
//             receiver_endpoint: endpoint,
//             buffers: bufs,
//             buffer_capacity: buf_capacity,
//             client: cli,
//         }
//     }

//     /// A custom receive method which will receive messages and push them to the buffer. If the buffer reaches capacity,
//     /// then it will call upload_data() to push the buffer's messages to AWS.
//     pub async fn receive_data(&mut self) {
//         loop {
//             match self.receiver_endpoint.recv() {
//                 Ok(data) => {self.filter_buffer(data).await},
//                 Err(e) => println!("Unable to receive data: {:?}", e),
//             }
//             println!("Working"); // for testing
//         }
//     }

//     /// A separate function that sorts the datapackets and pushes it to the appropriate buffer. If the buffer is
//     /// full, it will instead upload the data to AWS, clear the buffer, then push to the buffer.
//     async fn filter_buffer(&mut self, data: DataPacket) {
//         let buffer = match (data.exchange.as_str(), data.channel.as_str()) {
//             ("Binance", "Market") => &mut self.buffers.binance_market,
//             ("Binance", "Trade") => &mut self.buffers.binance_trade,
//             ("Huobi", "Market") => &mut self.buffers.huobi_market,
//             ("Huobi", "Trade") => &mut self.buffers.huobi_trade,
//             _ => return,
//         };

//         if buffer.len() < self.buffer_capacity {
//             buffer.push(data);
//         } else {
//             // Buffers::upload_data(buffer).await;
//             println!("Uploaded data!");
//             buffer.clear();
//             buffer.push(data);
//         }
//     }
// }

// impl Buffers {
//     /// A method that will upload data to AWS. It contains checks to ensure that there is an existing Timestream
//     /// database and table, and will create them if necessary. After uploading data, the buffer will be cleared so
//     /// future messages can be added.
//     pub async fn upload_data() { //buffer: &mut Vec<DataPacket> is the parameters, add back after done testing
//         println!("1");
//         let region = Region::new("us-east-1");
//         println!("1.5");
//         let config = SdkConfig::builder().region(region).build();
//         println!("2");
//         // let config = aws_config::load_from_env().await;
//         let client = AWSClient::new(&config);
//         println!("3");

//         let common_attributes = Record::builder()
//             .measure_name("cpu_usage")
//             .dimensions(Dimension::builder().name("host").value("host1").build())
//             .build();

//         let new_record = Record::builder()
//             .measure_name("cpu_usage")
//             .measure_value("13.5")
//             .time(chrono::Utc::now().to_rfc3339())
//             .dimensions(Dimension::builder().name("host").value("host1").build())
//             .build();
//         println!("4");

//         let _request = client
//             .write_records()
//             .database_name("sampleDB")
//             .table_name("sampleTB")
//             .common_attributes(common_attributes)
//             .records(new_record)
//             .send()
//             .await
//             .unwrap_or_else(|e| {
//                 println!("Error: {}", e);
//                 panic!("Error sending data to timestream");
//             });
//         println!("5");
//     }
// }
