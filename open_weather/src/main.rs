extern crate hyper;
mod weather;

use serde_json;
use std::{fs, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load config
    let contents = fs::read_to_string("config.json")?;
    let cfg: serde_json::Value = serde_json::from_str(&contents)?;

    // Load args
    let args: Vec<String> = env::args().collect();

    // Load and print data
    let api = weather::OpenWeatherApi {api_key: cfg["API_KEY"].to_string().replace("\"", "")};
    let json = api.get_json(args[1].to_string()).await;
    let raw_json = format!(r#"{}"#, json[10..json.len() - 3].to_string().replace("\\", ""));
    let v: serde_json::Value = serde_json::from_str(&raw_json)?;
    api.print_data(v);
    
    Ok(())
}