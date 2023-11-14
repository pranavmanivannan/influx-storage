use crate::data::{AWSData, DataEnum, DataPacket};
use chrono::Utc;
use hex;
// use influxdb::{Client, Error, InfluxDbWriteable, ReadQuery, Timestamp};
use reqwest;
use ring::{
    digest,
    hmac::{self, Tag},
    signature,
};
use serde_json::json;
use std::{
    str::FromStr,
    sync::{self, mpsc}, env, path::Path,
};

use aws_sdk_s3::{self, primitives::ByteStream};

// const ALL_ACCESS: &str = "8YpDls5dRzdIKzQ0ZKtKhb7_seSW3cCZeFEZpcIxBn0HOoNm12RCkG_9KMSfAyVaSJw--fBb3SlanVwdwKInrw==";
const ACCESS: &str = "uwQk-ij3njeKBiBsOr2Q6viFUQAZ0fPqVzWeniMG2OpgKoZcnsVLPZKkv7aXHDj79cG0vJp2q-1xS2OfEB08hQ==";

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

    /// A separate function that sorts the datapackets and pushes it to the appropriate buffer. If the buffer is
    /// full, it will instead upload the data to AWS, clear the buffer, then push to the buffer.
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
            exchange: data.exchange,
            json: data_message,
            time: Utc::now(),
        };

        buffer.push(aws_data);

        if buffer.len() > self.buffer_capacity {
            let url = "https://example.com/upload";
            let data = serde_json::to_string(buffer).expect("Failed to serialize buffer to JSON"); // make it so that it doesnt panic

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


impl Buffers {
    pub async fn write_data() {

    }

    pub async fn query_data() {

    }

    pub async fn get_bucket() {

    }


    pub async fn create_bucket() -> Result<(), reqwest::Error>{
        let influxdb_url = "http://localhost:8086";
        // let influxdb_url = "https://us-east-1-1.aws.cloud2.influxdata.com";
        let organization = "pranavm";
        // let organization = "400829963c428647";
        let token = ACCESS;

        let bucket_name = "binance";
        let create_bucket_url = format!("{}/api/v2/write?org={}&bucket={}&precision=ns", influxdb_url, organization, bucket_name);

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



// impl Buffers {
//     // pub async fn query(&self) {
//     //     let action = "Query";
//     //     let action_params = "";
//     //     let date = Utc::now();
//     //     let query = "SELECT * FROM \"binance\".\"IoT\" LIMIT 10";
//     //     let cli = reqwest::Client::new();
//     //     let auth_header = self.make_auth_header("".to_string());
//     //     let resp = cli
//     //         .get("https://query.timestream.us-east-1.amazonaws.com")
//     //         .header("Authorization", auth_header)
//     //         .body(query)
//     //         .send()
//     //         .await;
//     //     println!("{:?}", resp.as_ref().unwrap());
//     // }

//     /// Creates an Authorization header to be used for requests to the AWS API
//     fn make_auth_header(&self, payload: String) -> String {
//         let region = "us-east-1".to_string();
//         let service = "timestream".to_string();
//         let access_key = "".to_string();
//         let utc = Utc::now();

//         let canonical_request = self.canonical_request("GET".to_string(), payload);
//         let string_to_sign_ =
//             self.string_to_sign(canonical_request, region.clone(), service.clone());
//         let signature = self.calc_signature(string_to_sign_);

//         let algorithm = "Authorization: AWS4-HMAC-SHA256";
//         let credential = format!(
//             "Credential={}/{}/{}/{}/aws4_request,",
//             access_key,
//             utc.format("%Y%m%d").to_string(),
//             region,
//             service
//         );
//         let signed_headers = "SignedHeaders=host;x-amz-date,";
//         let sign = format!("Signature={}", signature);

//         let result = format!("{}{}{}{}", algorithm, credential, signed_headers, sign);

//         result
//     }

//     /// Creates a canonical request for signing an AWS API request
//     fn canonical_request(&self, method: String, payload: String) -> String {
//         let canonical_uri = "";
//         let canonical_query_string = "";
//         let canonical_headers = "";
//         let signed_headers = "";
//         let mut hashed_payload: String = self.hex(self.sha256hash("".to_string()));
//         if payload != "" {
//             hashed_payload = self.hex(self.sha256hash(payload));
//         }

//         let result: String = format!(
//             "{}{}{}{}{}{}",
//             method,
//             canonical_uri,
//             canonical_query_string,
//             canonical_headers,
//             signed_headers,
//             hashed_payload
//         );

//         result
//     }

//     /// Creates a string to sign using the canonical request
//     fn string_to_sign(&self, canonical_request: String, region: String, service: String) -> String {
//         let utc = Utc::now();

//         let algorithm = "AWS4-HMAC-SHA256\n";
//         let request_date_time = format!("{}\n", utc.to_rfc3339());
//         let credential_scope = format!(
//             "{}/{}/{}/aws4_request\n",
//             utc.format("%Y%m%d").to_string(),
//             region,
//             service
//         );
//         let hashed_canonical_request = self.hex(self.sha256hash(canonical_request));

//         let result: String = format!(
//             "{}{}{}{}",
//             algorithm, request_date_time, credential_scope, hashed_canonical_request
//         );

//         result
//     }

//     /// Calculates the signature to use for the signed request
//     fn calc_signature(&self, string_to_sign_: String) -> String {
//         let secret_access_key = "";
//         let kdate = self.hmac_sh256(
//             "AWS4".to_string() + secret_access_key,
//             Utc::now().format("%Y%m%d").to_string(),
//         );
//         let kregion = self.hmac_sh256(kdate, "us-east-1".to_string());
//         let kservice = self.hmac_sh256(kregion, "timestream".to_string());
//         let ksigning = self.hmac_sh256(kservice, "aws4_request".to_string());

//         let signature = self.hmac_sh256(ksigning, string_to_sign_);

//         self.hex(signature)
//     }

//     /// Converts a string to lowercase
//     fn lowercase(&self, string: String) -> String {
//         string.to_lowercase()
//     }

//     /// Returns the lowercase base 16 encoding
//     fn hex(&self, input: String) -> String {
//         hex::encode(input)
//     }

//     /// Secure Hash Algorithm (SHA) cryptographic hash function
//     fn sha256hash(&self, input: String) -> String {
//         let digest = ring::digest::digest(&digest::SHA256, input.as_bytes());
//         let hash_bytes = digest.as_ref();
//         let hash_hex: String = hash_bytes
//             .iter()
//             .map(|byte| format!("{:02x}", byte))
//             .collect();
//         hash_hex
//     }

//     /// Computes HMAC by using the SHA256 algorithm with the signing key provided
//     fn hmac_sh256(&self, signing_key: String, input: String) -> String {
//         let hmac_key = ring::hmac::Key::new(hmac::HMAC_SHA256, signing_key.as_bytes());
//         ring::hmac::sign(&hmac_key, input.as_bytes())
//             .as_ref()
//             .iter()
//             .map(|byte| format!("{:02x}", byte))
//             .collect()
//     }
// }
