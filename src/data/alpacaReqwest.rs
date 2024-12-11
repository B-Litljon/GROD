use reqwest::{blocking::Client, Error, header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct AlpacaClient {
    base_url: String,
    api_key: String,
    api_secret: String,
    endpoint: String,
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

    pub fn get_static_candles(
        &self,
        symbol_or_symbols: Vec<String>, //array of ticker symbols
        timeframe: String, //the timeframe for the request. eg: 1Min, 1(H)our, 1(D)ay, 
        start: String,
        end: String,
        limit: Option<i32>, // limit for the amount of data ot be received 
    ) -> Result<HashMap<String, serde_json::Value>, Error> {
        // format(base_url/{}) 
        // concat the base url with the endpoint you want to request
        let url = format!("{}/stock/bars", self.base_url);
        
        // creating the query parameters using a hashmap 
        let mut query_params = HashMap::new();
        query_params.insert("symbol_or_symbols", symbol_or_symbols.join(","));
        query_params.insert("timeframe", timeframe);
        query_params.insert("start", start);
        query_params.insert("end", end);
        if let Some(l) = limit {
            query_params.insert("limit", l.to_string());
        }

        let response = self
            .client
            .get(url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .query(&query_params)
            .send()?;
        
        response.json::<HashMap<String, serde_json::Value>>()
    }
}