use reqwest::Client;
use serde_json::Value;

pub async fn post(url: &str, payload: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response: Value = client.post(url).json(&payload).send().await?.json().await?;

    Ok(response)
}
