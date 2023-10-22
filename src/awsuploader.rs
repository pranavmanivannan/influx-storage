use hyper::Client;
use std::sync::mpsc;

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
    ) -> AWSUploader<T> {
        AWSUploader {
            receiver_endpoint: endpoint,
            buffer: buf,
            buffer_capacity: buf_capacity,
            database_name: db_name,
            table_name: tb_name,
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

        // if spawning a client is expensive, we could spawn one in main and pass in clones to each awsuploader made
        // so that we can make all calls on one client
        // if one client can't handle the requests, we may need to performance test to see how many clients are needed
        // to be optimal for our use case (maybe one per exchange? one per channel seems too much)
        let client = Client::new();

        let resp = client
            .get("http://example.com".parse().unwrap());

        println!("Response: {:?}", resp);

        self.buffer.clear();
    }
}
