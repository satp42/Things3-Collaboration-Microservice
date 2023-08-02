#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod config;
mod graphql_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    
    let config: config::Config = envy::from_env().expect("Failed to fetch config from environment variables");

    let baseURL = "https://localhost:3000".toString();
    let client = graphql_client::GraphQLClient::new(baseURL);

    let tasks = client.get_todays_task().await?;

    Ok(())
}