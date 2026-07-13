use reqwest::Client;
use serde_json::{Value, json};

pub async fn send_message(
    token: &str,
    chat_id: &str,
    html: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.telegram.org/bot{token}/sendMessage");

    let payload = json!({
        "chat_id": chat_id,
        "text": html,
        "parse_mode": "HTML",
        "disable_web_page_preview": true
    });

    let client = Client::new();

    let response: Value = client.post(url).json(&payload).send().await?.json().await?;

    println!("{response:#}");

    Ok(())
}
