use std::{sync::OnceLock, time::Duration};

use reqwest::{Client, Response};
use serde_json::Value;

const REQUEST_TIMEOUT_SECS: u64 = 10;
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn post(url: &str, payload: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let response: Response = client()
        .post(url)
        .json(&payload)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .send()
        .await?
        .error_for_status()?;

    let json: Value = response.json().await?;

    Ok(json)
}

fn client() -> &'static Client {
    return HTTP_CLIENT.get_or_init(Client::new);
}
