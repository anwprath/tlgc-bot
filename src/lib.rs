use reqwest::Client;
use serde_json::{json, Value};
use worker::*;

#[event(fetch)]
async fn fetch(_req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response> {
    let result = fetch_question()
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    let bot_token = _env
        .var("LEETCODE_CLOWN_BOT_TOKEN")
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    let chat_id = _env
        .var("CHAT_ID")
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    let html = result
        .get("data")
        .and_then(|v| v.get("activeDailyCodingChallengeQuestion"))
        .and_then(|v| v.get("question"))
        .and_then(|v| v.get("content"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| worker::Error::RustError("content not found".to_string()))?;

    let htmlText = html2text::from_read(html.as_bytes(), 80)
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    println!("{}", result);
    let respnse = send_message(
        &bot_token.to_string(),
        &chat_id.to_string(),
        &htmlText.into_boxed_str(),
    )
    .await
    .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Response::from_json(&respnse)
}

pub async fn fetch_question() -> Result<Value, Box<dyn std::error::Error>> {
    let query = r#"
    query questionOfToday {
            activeDailyCodingChallengeQuestion {
                date
                question {
                    questionFrontendId
                    title
                    titleSlug
                    difficulty
                    content
                    topicTags { name }
                }
            }
        }"#;

    let resp = graphql(query, json!({})).await?;

    Ok(resp)
}

async fn graphql(query: &str, variables: Value) -> Result<Value, Box<dyn std::error::Error>> {
    const GRAPHQL_URL: &str = "https://leetcode.com/graphql";

    let payload = json!({
        "query": query,
        "variables": variables
    });

    let client = Client::new();

    let response: Value = client
        .post(GRAPHQL_URL)
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;

    println!("{response:#}");
    Ok(response)
}

pub async fn send_message(
    token: &str,
    chat_id: &str,
    html: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("https://api.telegram.org/bot{token}/sendMessage");

    let payload = json!({
        "chat_id": chat_id,
        "text": escape_md(html),
          "parse_mode": "Markdown",
        "disable_web_page_preview": true
    });

    let client = Client::new();

    let response: Value = client.post(url).json(&payload).send().await?.json().await?;

    println!("{response:#}");

    Ok(response)
}

fn escape_md(text: &str) -> String {
    let special = r#">#+-=|{}.!"#;

    let mut out = String::new();
    for c in text.chars() {
        if special.contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}
