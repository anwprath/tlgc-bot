use std::env;

use reqwest::Client;
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    send_message(&env::var("LEETCODE_CLOWN_BOT_TOKEN")?.to_string(), &env::var("CHAT_ID")?.to_string(), "
<b>🚀 System Alert: Daily Report</b>

Attention <i>Admin</i>, the scheduled backup for <u>Database_Main</u> is now <b><ins>COMPLETE</ins></b>. 

<b>Summary of Events:</b>
• Total files processed: <code>14,250</code>
• Execution time: <tg-spoiler>0.42 seconds</tg-spoiler>
• Status code: <a href=\"https://httpstatuses.com\">200 OK</a>

🤖 <i>Please review the logs if you notice any latency.</i>
").await?;
    Ok(())
}

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
