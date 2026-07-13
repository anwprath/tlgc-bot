use std::env;

mod telegram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    telegram::send_message(
        &env::var("LEETCODE_CLOWN_BOT_TOKEN")?.to_string(),
        &env::var("CHAT_ID")?.to_string(),
        "<b>🚀 System Alert: Daily Report</b>",
    )
    .await?;
    Ok(())
}
