use std::io;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::httpw;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    let response = httpw::post(GRAPHQL_URL, payload).await?;
    parse_question_response(response)
}

fn parse_question_response(response: Value) -> Result<Question, Box<dyn std::error::Error>> {
    let graphql: GraphqlResponse = serde_json::from_value(response)?;
    let question = graphql
        .data
        .and_then(|data| data.active_daily_coding_challenge_question)
        .and_then(|challenge| challenge.question)
        .ok_or_else(|| io::Error::other("LeetCode response missing question payload"))?;

    let content = question
        .content
        .ok_or_else(|| io::Error::other("LeetCode response missing question content"))?;
    let title_slug = question
        .title_slug
        .ok_or_else(|| io::Error::other("LeetCode response missing title slug"))?;
    let link = format!("https://leetcode.com/problems/{title_slug}/description/");

    Ok(Question { content, link })
}

#[derive(Deserialize)]
struct GraphqlResponse {
    data: Option<GraphqlData>,
}

#[derive(Deserialize)]
struct GraphqlData {
    #[serde(rename = "activeDailyCodingChallengeQuestion")]
    active_daily_coding_challenge_question: Option<DailyChallenge>,
}

#[derive(Deserialize)]
struct DailyChallenge {
    question: Option<LeetcodeQuestion>,
}

#[derive(Deserialize)]
struct LeetcodeQuestion {
    content: Option<String>,
    #[serde(rename = "titleSlug")]
    title_slug: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{parse_question_response, Question};

    #[test]
    fn parses_question_payload() {
        let response = json!({
            "data": {
                "activeDailyCodingChallengeQuestion": {
                    "question": {
                        "content": "<p>Question text</p>",
                        "titleSlug": "two-sum"
                    }
                }
            }
        });

        let question = parse_question_response(response).expect("expected valid question");

        assert_eq!(
            question,
            Question {
                content: "<p>Question text</p>".to_string(),
                link: "https://leetcode.com/problems/two-sum/description/".to_string()
            }
        );
    }

    #[test]
    fn errors_when_payload_missing_fields() {
        let response = json!({
            "data": {
                "activeDailyCodingChallengeQuestion": {
                    "question": {
                        "content": "<p>Missing slug</p>"
                    }
                }
            }
        });

        let result = parse_question_response(response);

        assert!(result.is_err());
    }
}
