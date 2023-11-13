use aws_config::meta::region::RegionProviderChain;
use aws_sdk_apigateway as apigateway;

pub struct APIUploader {
    client: apigateway::Client,
}

impl APIUploader {
    pub fn new(client: apigateway::Client) -> Self {
        APIUploader { client }
    }

    pub async fn get_rest_apis(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self.client.get_rest_apis().send().await?;

        for api in resp.items() {
            println!("ID:          {}", api.id().unwrap_or_default());
            println!("Name:        {}", api.name().unwrap_or_default());
            println!("Description: {}", api.description().unwrap_or_default());
            println!("Version:     {}", api.version().unwrap_or_default());
            println!();
        }

        Ok(())
    }
}
