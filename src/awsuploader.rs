// use aws_config::SdkConfig;
// use aws_sdk_timestreamquery::types::DimensionValueType;
// use aws_sdk_timestreamwrite::{
//     types::{Dimension, Record},
//     Client as AWSClient,
// };
// // use aws_sdk_timestreamwrite::model::Record::CommonAttributes;
// use aws_types::region::Region;
// use hyper::{Client, Request, Response, Uri};
use std::{str::FromStr, sync::{self, mpsc}};
use crate::data::{AWSData, DataEnum, DataPacket};
use chrono::Utc;
use hex;
use reqwest;
use ring::{
    digest,
    hmac::{self, Tag}, signature,
};

/// A struct for setting a channel receiver endpoint and uploading the messages to AWS services.
pub struct AWSUploader {
    pub client: reqwest::Client,
    pub receiver_endpoint: mpsc::Receiver<DataPacket>,
    pub buffers: Buffers,
    pub buffer_capacity: usize,
}

/// A struct for keeping track of all buffers needed to store and upload data to AWS.
pub struct Buffers {
    pub binance_market: Vec<AWSData>,
    pub binance_trade: Vec<AWSData>,
    pub huobi_market: Vec<AWSData>,
    pub huobi_trade: Vec<AWSData>,
}

/// An implementation of AWSUploader with a constructor alongside methods for receiving messages from a channel
/// and uploading a buffer to AWS.
///
impl AWSUploader {
    /// Basic constructor for AWSUploader that takes in a Receiver<DataPacket> endpoint, a Buffers struct to hold the
    /// buffers it will store messages in, and a buffer capacity.
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

    //test get endpoint
    pub async fn get(&self) -> Result<String, reqwest::Error> {
        let url = "https://w6fxw1zkgg.execute-api.us-east-1.amazonaws.com/samplestage/pets";
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;

        Ok(body)
    }

    // A custom receive method which will receive messages and push them to the buffer. If the buffer reaches capacity,
    // then it will call upload_data() to push the buffer's messages to AWS.
    pub async fn receive_data(&mut self) {
        loop {
            match self.receiver_endpoint.recv() {
                Ok(data) => self.filter_buffer(data).await,
                Err(e) => println!("Unable to receive data: {:?}", e),
            }
            println!("Working"); // for testing
        }
    }

    // A separate function that sorts the datapackets and pushes it to the appropriate buffer. If the buffer is
    // full, it will instead upload the data to AWS, clear the buffer, then push to the buffer.
    async fn filter_buffer(&mut self, data: DataPacket) {
        let buffer: &mut Vec<AWSData> = match (data.exchange.as_str(), data.channel.as_str()) {
            ("Binance", "Market") => &mut self.buffers.binance_market,
            ("Binance", "Trade") => &mut self.buffers.binance_trade,
            ("Huobi", "Market") => &mut self.buffers.huobi_market,
            ("Huobi", "Trade") => &mut self.buffers.huobi_trade,
            _ => return,
        };

        let data_message = match data.data {
            DataEnum::M1(msg) => msg.data.clone(),
            DataEnum::M2(msg) => msg.best_ask.clone(),
        };

        let aws_data = AWSData {
            json: data_message,
            timestamp: Utc::now(),
        };

        buffer.push(aws_data);

        if buffer.len() > self.buffer_capacity {
            let url = "https://example.com/upload";
            let data = serde_json::to_string(buffer).expect("Failed to serialize buffer to JSON"); // need to make it so that it doesnt panic

            let response = self
                .client
                .post(url)
                .header("Content-Type", "application/json")
                .body(data)
                .send()
                .await;

            buffer.clear();
        }
    }
}


impl Buffers {
    pub async fn upload() {
        let action = "Query";
        let action_params = "";
        let date = Utc::now();
        let query = "SELECT * FROM \"binance\".\"IoT\" LIMIT 10";
        let cli = reqwest::Client::new();
        let resp = cli
            .post("https://query.timestream.us-east-1.amazonaws.com")
            .body(query)
            .send()
            .await;
        println!("{:?}", resp.as_ref().unwrap());
    }

    pub async fn query(&self) {
        let query_req = self.make_query_request("".to_string());
        let cli = reqwest::Client::new();
        let resp = cli.get(query_req).send().await;
        println!("{:?}", resp);
    }

    fn make_query_request(&self, payload: String) -> String {
        let region = "us-east-1".to_string();
        let service = "timestream".to_string();
        let access_key = "".to_string();
        let utc = Utc::now();

        let canonical_request = self.canonical_request("GET".to_string(), payload);
        let string_to_sign_ = self.string_to_sign(canonical_request, region.clone(), service.clone());
        let signature = self.calc_signature(string_to_sign_);

        let request = "https://query.timestream.us-east-1.amazonaws.com/?Action=Query&Version=2023-11-14";
        let algorithm = "X-Amz-Algorithm=AWS4-HMAC-SHA256&";
        let credential = format!("X-Amz-Credential={}/{}/{}/{}/aws4_request&", access_key, utc.format("%Y%m%d").to_string(), region, service);
        let date = format!("X-Amz-Date={}&", utc.to_rfc3339());
        let signed_headers = "X-Amz-SignedHeaders=host;x-amz-date&";
        let sign = format!("X-Amz-Signature={}", signature);

        let result = format!("{}{}{}{}{}{}", request, algorithm, credential, date, signed_headers, sign);

        result
    }

    fn canonical_request(&self, method: String, payload: String) -> String {
        let canonical_uri = "";
        let canonical_query_string = "";
        let canonical_headers = "";
        let signed_headers = "";
        let mut hashed_payload: String = self.hex(self.sha256hash("".to_string()));
        if payload != "" {
            hashed_payload = self.hex(self.sha256hash(payload));
        }

        let result: String = format!("{}{}{}{}{}{}", method, canonical_uri, canonical_query_string, canonical_headers, signed_headers, hashed_payload);

        result
    }

    fn string_to_sign(&self, canonical_request: String, region: String, service: String) -> String {
        let utc = Utc::now();

        let algorithm = "AWS4-HMAC-SHA256\n";
        let request_date_time = format!("{}\n", utc.to_rfc3339());
        let credential_scope = format!("{}/{}/{}/aws4_request\n", utc.format("%Y%m%d").to_string(), region, service);
        let hashed_canonical_request = self.hex(self.sha256hash(canonical_request));
        
        let result: String = format!("{}{}{}{}", algorithm, request_date_time, credential_scope, hashed_canonical_request);

        result
    }

    fn calc_signature(&self, string_to_sign_: String) -> String {
        let secret_access_key = "dfd";
        let kdate = self.hmac_sh256("AWS4".to_string() + secret_access_key, Utc::now().format("%Y%m%d").to_string());
        let kregion = self.hmac_sh256(kdate, "us-east-1".to_string());
        let kservice = self.hmac_sh256(kregion, "timestream".to_string());
        let ksigning = self.hmac_sh256(kservice, "aws4_request".to_string());
        
        let signature = self.hmac_sh256(ksigning, string_to_sign_);

        self.hex(signature)
    }

    fn lowercase(&self, string: String) -> String {
        string.to_lowercase()
    }

    fn hex(&self, input: String) -> String {
        hex::encode(input)
    }

    fn sha256hash(&self, input: String) -> String {
        let digest = ring::digest::digest(&digest::SHA256, input.as_bytes());
        let hash_bytes = digest.as_ref();
        let hash_hex: String = hash_bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();
        hash_hex
    }

    fn hmac_sh256(&self, signing_key: String, input: String) -> String {
        let hmac_key = ring::hmac::Key::new(hmac::HMAC_SHA256, signing_key.as_bytes());
        ring::hmac::sign(&hmac_key, input.as_bytes()).as_ref().iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}




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
