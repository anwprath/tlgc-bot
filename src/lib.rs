use ammonia::Builder;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
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

    println!("{}", result);
    let respnse = send_message(&bot_token.to_string(), &chat_id.to_string(), html)
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
        "text": sanitize_for_telegram(html),
        "parse_mode": "HTML",
        "disable_web_page_preview": true
    });

    let client = Client::new();

    let response: Value = client.post(url).json(&payload).send().await?.json().await?;

    println!("{response:#}");

    Ok(response)
}

fn telegram_sanitizer() -> Builder<'static> {
    let mut builder = Builder::default();

    let tags: HashSet<&str> = [
        "b",
        "strong",
        "i",
        "em",
        "u",
        "ins",
        "s",
        "strike",
        "del",
        "span",
        "tg-spoiler",
        "a",
        "tg-emoji",
        "code",
        "pre",
        "blockquote",
    ]
    .into_iter()
    .collect();

    let mut tag_attributes: HashMap<&str, HashSet<&str>> = HashMap::new();
    tag_attributes.insert("a", ["href"].into_iter().collect());
    tag_attributes.insert("span", ["class"].into_iter().collect());
    tag_attributes.insert("tg-emoji", ["emoji-id"].into_iter().collect());
    tag_attributes.insert("code", ["class"].into_iter().collect()); // language-xxx
    tag_attributes.insert("blockquote", ["expandable"].into_iter().collect());
    tag_attributes.insert("span", ["class"].into_iter().collect());

    builder
        .tags(tags)
        .tag_attributes(tag_attributes)
        .set_tag_attribute_value("span", "class", "tg-spoiler")
        .link_rel(None) // don't inject rel="noopener" etc — Telegram doesn't want extra attrs
        .clean_content_tags(HashSet::new());

    builder
}

fn sanitize_for_telegram(input: &str) -> String {
    telegram_sanitizer().clean(input).to_string()
}
