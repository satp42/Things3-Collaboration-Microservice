#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

// Config struct will go here

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    
    // Getting config from env variables will go here
    
    // Fetching tasks and sending SMS/Email functionalities will be placed here

    Ok(())
}