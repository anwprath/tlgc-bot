use reqwest::Client;
use serde_json::Value;
use std::{sync::OnceLock, time::Duration};

const REQUEST_TIMEOUT_SECS: u64 = 20;
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

fn client() -> &'static Client {
    HTTP_CLIENT.get_or_init(Client::new)
}

pub async fn post(url: &str, payload: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let response = client()
        .post(url)
        .json(&payload)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .send()
        .await?
        .error_for_status()?;
    let response: Value = response.json().await?;

    Ok(response)
}
