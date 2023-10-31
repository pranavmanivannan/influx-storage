use dotenv::dotenv;
use hyper::{Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;
use rusoto_core::Region;
use rusoto_credential::DefaultCredentialsProvider;
use rusoto_credential::InstanceMetadataProvider;
use rusoto_credential::ProfileProvider;
use rusoto_credential::ProvideAwsCredentials;
use rusoto_credential::StaticProvider;
use rusoto_signature::SignedRequest;
use serde::Serialize;
use std::env;
use std::error::Error;
use std::f32::consts::E;
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
    pub client: Client<hyper::client::HttpConnector>,
}

/// An implementation of AWSUploader with a constructor alongside methods for receiving messages from a channel
/// and uploading a buffer to AWS.
impl<T> AWSUploader<T>
where
    T: data::Data + Serialize,
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
    pub async fn receive_data(&mut self) {
        println!("called receive_data");
        loop {
            match self.receiver_endpoint.recv() {
                Ok(data) => self.buffer.push(data),
                Err(e) => println!("Unable to receive data: {:?}", e),
            }
            println!("Working"); // for testing
            if self.buffer.len() > self.buffer_capacity {
                self.upload_data().await;
                println!("Uploaded data!");
                println!("{:?}", self.buffer.len());
            }
        }
    }

    /// A method that will upload data to AWS. It contains checks to ensure that there is an existing Timestream
    /// database and table, and will create them if necessary. After uploading data, the buffer will be cleared so
    /// future messages can be added.
    async fn upload_data(&mut self) {
        // user hyper.rs for the following steps:
        // 1) check if database exists, if not make one (use self.database)
        // 2) check if table in database exists, if not make one (use self.table)
        // 3) upload data to AWS

        // if spawning a client is expensive, we could spawn one in main and pass in clones to each awsuploader made
        // so that we can make all calls on one client
        // if one client can't handle the requests, we may need to performance test to see how many clients are needed
        // to be optimal for our use case (maybe one per exchange? one per channel seems too much)
        println!("1");
        // adds endpoint variables
        let aws_endpoint_url: &str = "https://ingest.timestream.us-east-1.amazonaws.com";
        let database_name = self.database_name.clone();
        let table_name = self.table_name.clone();

        println!("2");
        // initializes AWS credentials
        dotenv().ok();
        let aws_access_key = "AKIASUUPPFM2GQVZVAF4".to_owned();
        let aws_secret_key: String = "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".to_owned();

        println!("3");
        // creates an HTTPS client
        let https = HttpsConnector::new();
        let client: Client<HttpsConnector<hyper::client::HttpConnector>> =
            Client::builder().build::<_, Body>(https);
        // let client: Client<hyper::client::HttpConnector>

        println!("4");
        // checks and create the database
        self.check_and_create_database(
            &client,
            &aws_access_key,
            &aws_secret_key,
            aws_endpoint_url,
            &database_name,
        )
        .await;

        println!("5");
        // checks and create the table
        self.check_and_create_table(
            &client,
            &aws_access_key,
            &aws_secret_key,
            aws_endpoint_url,
            &database_name,
            &table_name,
        )
        .await;

        println!("6");
        // inserts data into the specified table

        let serialized_data = match serde_json::to_string(&self.buffer.last()) {
            Ok(data) => data,
            Err(_) => todo!(),
        };
        let query = format!(
            "INSERT INTO {}.{} VALUE '{}'",
            database_name, table_name, serialized_data
        );

        println!("7");
        // Sign and send the query
        self.sign_and_send_query(
            &client,
            &aws_access_key,
            &aws_secret_key,
            aws_endpoint_url,
            query,
        )
        .await;

        // Ok(())
        self.buffer.clear();
    }

    // function to check if a database exists and create it if not
    async fn check_and_create_database(
        &mut self,
        client: &Client<HttpsConnector<hyper::client::HttpConnector>>,
        aws_access_key: &str,
        aws_secret_key: &str,
        aws_endpoint_url: &str,
        database_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        // Check if the database exists
        let check_query = format!("SHOW DATABASES LIKE '{}'", database_name);
        println!("Inside check and create db!");
        // Sign and send the query
        let check_response: bool = self
            .sign_and_send_query(
                &client,
                aws_access_key,
                aws_secret_key,
                aws_endpoint_url,
                check_query,
            )
            .await?;

        println!("DB1");

        // checks if the database exists and create it if not
        if check_response == true {
            // Implement AWS Signature Version 4
            let create_query = self
                .sign_query(
                    format!("CREATE DATABASE {}", database_name),
                    aws_access_key,
                    aws_secret_key,
                    aws_endpoint_url,
                )
                .await?;

            // Sign and send the query
            self.sign_and_send_query(
                &client,
                aws_access_key,
                aws_secret_key,
                aws_endpoint_url,
                create_query,
            )
            .await?;

            println!("Created database: {}", database_name);
        } else {
            println!("Database {} already exists.", database_name);
        }

        Ok(())
    }

    // Function to check if a table exists and create it if not
    async fn check_and_create_table(
        &mut self,
        client: &Client<HttpsConnector<hyper::client::HttpConnector>>,
        aws_access_key: &str,
        aws_secret_key: &str,
        aws_endpoint_url: &str,
        database_name: &str,
        table_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        // Check if table exists
        let create_query = format!("SHOW DATABASES LIKE '{}'", database_name);
        let query_clone_check = create_query.clone();
        // Sign and send the query
        let check_response = self
            .sign_and_send_query(
                client,
                aws_access_key,
                aws_secret_key,
                aws_endpoint_url,
                query_clone_check,
            )
            .await?;

        // Check if the table exists and create it if not
        if check_response == true {
            let query_clone_check = create_query.clone();
            // Sign and send the query
            self.sign_and_send_query(
                client,
                aws_access_key,
                aws_secret_key,
                aws_endpoint_url,
                query_clone_check,
            )
            .await?;

            println!("Created table: {}.{}", database_name, table_name);
        } else {
            println!("Table {}.{} already exists.", database_name, table_name);
        }

        Ok(())
    }

    // Signs and sends http requests
    async fn sign_and_send_query(
        &mut self,
        client: &Client<HttpsConnector<hyper::client::HttpConnector>>,
        aws_access_key: &str,
        aws_secret_key: &str,
        aws_endpoint_url: &str,
        query: String,
    ) -> Result<bool, Box<dyn Error>> {
        println!("sasq1");
        let copy: String = query.clone();
        let signed_query = self
            .sign_query(copy, aws_access_key, aws_secret_key, aws_endpoint_url)
            .await?;
        println!("sasq2");
        let cloned_query = signed_query.clone();
        println!("{}", cloned_query);
        match self
            .send_http_request(&client, aws_endpoint_url, cloned_query.to_owned())
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                eprintln!("Error sending HTTP request: {:?}", err);
                println!("Error sending HTTP request: {:?}", err);
                Ok(false)
            }
        }
    }
    async fn send_http_request(
        &mut self,
        client: &Client<HttpsConnector<hyper::client::HttpConnector>>,
        aws_endpoint_url: &str,
        query: String,
    ) -> Result<String, Box<dyn Error>> {
        let uri = aws_endpoint_url.parse::<Uri>()?;
        // Create an HTTP POST request with the query as the request body
        println!("In http req function");
        let request = Request::builder()
            .method("POST")
            .uri(uri)
            .header("Content-Type", "application/x-amz-json-1.1")
            .body(Body::from(query))?;

        // Send the HTTP request and await the response
        let response = client.request(request).await?;

        // Check if the response is successful
        if !response.status().is_success() {
            println!(
                "HTTP request failed with status code: {}",
                response.status()
            );
            return Err(format!(
                "HTTP request failed with status code: {}",
                response.status()
            )
            .into());
        }

        // Read the response body as a string
        let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body_str = String::from_utf8(body_bytes.to_vec())?;

        Ok(body_str)
    }

    // signs the query to make authenticated requested
    async fn sign_query(
        &mut self,
        query: String,
        aws_access_key: &str,
        aws_secret_key: &str,
        aws_endpoint_url: &str,
    ) -> Result<String, Box<dyn Error>> {
        // //create a new SignedRequest
        // println!("sq1");
        // let mut request = SignedRequest::new(
        //     "POST",
        //     "timestream-query",
        //     &Region::UsEast1,
        //     aws_endpoint_url,
        // );
        // println!("sq2");
        // request.set_content_type("application/x-amz-json-1.1".to_owned());

        // let query_as_bytes = query.as_bytes().to_owned();
        // request.set_payload(Some(query_as_bytes));

        // println!("sq3");
        // // initializes AWS credentials provider
        // let credentials_provider = StaticProvider::new_minimal(
        //     "AKIASUUPPFM2GQVZVAF4".to_owned(),
        //     "mK5NGTqnBGBnOyj8P8XMPr482T3XXzt3bH3uizgz".to_owned(),
        // );
        // println!("sq3.1");
        // let credentials_result = credentials_provider.credentials().await;
        // let credentials = match credentials_result {
        //     Ok(credentials) => credentials,
        //     Err(err) => {
        //         eprintln!("Error getting credentials: {:?}", err);
        //         return Err(Box::new(err));
        //     }
        // };

        // println!("sq3.2");
        // // Clone the credentials
        // let cloned_credentials = credentials.clone();

        // println!("sq4");
        // // set AWS access key and secret key
        // request.add_header("X-Amz-Access-Key", &aws_access_key);
        // request.add_header("X-Amz-Secret-Key", &aws_secret_key);

        // // signs the request using AWS Signature Version 4
        // request.sign(&cloned_credentials);

        // // Generate the signed URL with query parameters
        // let signed_url: String = request.generate_presigned_url(
        //     &credentials,
        //     &std::time::Duration::from_secs(3600),
        //     None::<&str>.is_some(),
        // );
        // println!("sq5");

        Ok("https://ingest.timestream.us-east-1.amazonaws.com?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIASUUPPFM2GQVZVAF4%2F20231031%2Fus-east-1%2Ftimestream-query%2Faws4_request&X-Amz-Date=20231031T035337Z&X-Amz-Expires=3600&X-Amz-Signature=d52627f9b70f8487b92a95ce782aad47a979f39b36f1bda10da63388cc338b2c&X-Amz-SignedHeaders=host%3Bx-amz-access-key%3Bx-amz-secret-key".to_owned())
    }
}
