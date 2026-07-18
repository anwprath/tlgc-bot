use std::collections::{HashMap, HashSet};

use ammonia::Builder;
use serde_json::{json, Value};

use crate::{httpw, leetcode::Question};

const BASE_URL: &str = "https://api.telegram.org";

pub async fn send_message(
    token: &str,
    chat_id: &str,
    question: &Question,
) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("{BASE_URL}/bot{token}/sendMessage");

    let sanitized_html = sanitize_for_telegram(&question.content);
    let link_suffix = format!("\n\n{}", question.link);
    let text = format!("{sanitized_html}{link_suffix}");

    let payload = json!({
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "HTML",
        "disable_web_page_preview": true
    });

    httpw::post(&url, payload).await
}

// ------------------- START: AI (slur) Generated Section --------------------------
fn sanitize_for_telegram(input: &str) -> String {
    telegram_sanitizer().clean(input).to_string()
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

    builder
        .tags(tags)
        .tag_attributes(tag_attributes)
        .set_tag_attribute_value("span", "class", "tg-spoiler")
        .link_rel(None) // don't inject rel="noopener" etc — Telegram doesn't want extra attrs
        .clean_content_tags(HashSet::new());

    builder
}

// ------------------- END: AI (slur) Generated Section --------------------------
