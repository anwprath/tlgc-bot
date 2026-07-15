use serde_json::{json, Value};

use crate::httpw;

pub struct Question {
    pub content: String,
    pub link: String,
}

const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const QUERY: &str = r#"
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

pub async fn fetch_question() -> Result<Question, Box<dyn std::error::Error>> {
    let payload = json!({
        "query": QUERY,
        "variables": json!({})
    });

    let response = httpw::post(GRAPHQL_URL, payload).await;
    match response {
        Err(err) => Err(err),
        Ok(v) => Ok({
            let content = v
                .get("data")
                .and_then(|v| v.get("activeDailyCodingChallengeQuestion"))
                .and_then(|v| v.get("question"))
                .and_then(|v| v.get("content"))
                .and_then(|v| v.as_str())
                .or(Some("QuestionNotFound"));

            let title_slug = v
                .get("data")
                .and_then(|v| v.get("activeDailyCodingChallengeQuestion"))
                .and_then(|v| v.get("question"))
                .and_then(|v| v.get("titleSlug"))
                .and_then(|v| v.as_str())
                .or(Some("TitleSlugNotFound"));

            let link = format!(
                "https://leetcode.com/problems/{}/description/",
                title_slug.unwrap()
            );
            Question {
                content: content.unwrap().to_string(),
                link: link.to_owned(),
            }
        }),
    }
}
