use hyper::{Client, Request, Response, Uri};
use std::{str::FromStr, sync::mpsc};

use crate::data;

/// A struct for setting a channel receiver endpoint and uploading the messages to AWS services.
pub struct AWSUploader<T>
where
    T: data::Data,
{
    pub receiver_endpoint: mpsc::Receiver<T>,
    pub buffer: Vec<T>,
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
        endpoint: mpsc::Receiver<T>,
        buf: Vec<T>,
        buf_capacity: usize,
        db_name: String,
        tb_name: String,
        cli: Client<hyper::client::HttpConnector>,
    ) -> AWSUploader<T> {
        AWSUploader {
            receiver_endpoint: endpoint,
            buffer: buf,
            buffer_capacity: buf_capacity,
            database_name: db_name,
            table_name: tb_name,
            client: cli,
        }
    }

    /// A custom receive method which will receive messages and push them to the buffer. If the buffer reaches capacity,
    /// then it will call upload_data() to push the buffer's messages to AWS.
    pub fn receive_data(&mut self) {
        loop {
            match self.receiver_endpoint.recv() {
                Ok(data) => self.buffer.push(data),
                Err(e) => println!("Unable to receive data: {:?}", e),
            }
            println!("Working"); // for testing
            if self.buffer.len() > self.buffer_capacity {
                self.upload_data();
                println!("Uploaded data!");
            }
        }
    }

    /// A method that will upload data to AWS. It contains checks to ensure that there is an existing Timestream
    /// database and table, and will create them if necessary. After uploading data, the buffer will be cleared so
    /// future messages can be added.
    fn upload_data(&mut self) {
        // user hyper.rs for the following steps:
        // 1) check if database exists, if not make one (use self.database)
        // 2) check if table in database exists, if not make one (use self.table)
        // 3) upload data to AWS

        let db_url = "http://example.com";
        let db_request = self.create_request(db_url);
        let db_response = self.client.request(db_request);

        // check if
        let table_url = "http://example.com";
        let table_request = self.create_request(table_url);
        let table_response = self.client.request(table_request);

        // upload data to AWS
        let upload_url = "http://example.com";
        let upload_request = self.create_request(upload_url);
        let upload_response = self.client.request(upload_request);

        self.buffer.clear();
    }

    // need to do error handling here, probably
    fn create_request(&mut self, url: &str) -> hyper::Request<hyper::Body> {
        let uri = match Uri::from_str(url) {
            Ok(url) => url,
            Err(error) => {
                eprintln!("{:?}", error);
                Uri::from_static("error")
            }
        };

        let request = Request::builder()
            .method("GET")
            .uri(uri)
            .header("key", "value")
            .body(hyper::Body::empty())
            .unwrap(); // build request to check database, need to change to match case so this doesn't cause a panic!

        return request;
    }
}
