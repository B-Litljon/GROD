use reqwest::{blocking::Client, Error, header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AlpacaClient {
    base_url: String,
    api_key: String,
    api_secret: String,
    client: Client,
}

impl AlpacaClient {
    pub fn new(api_key: String, api_secret: String) -> Self {
        AlpacaClient {
            base_url: "https://data.alpaca.markets/v2".to_string(),
            api_key,
            api_secret,
            client: Client::new(),
        }
    }

    
}