use reqwest::Client;
use serde_json::{Value, json};

const GRAPHQL_URL: &str = "https://leetcode.com/graphql";

pub async fn fetch_question() -> Result<(), Box<dyn std::error::Error>> {
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

    graphql(query, json!({})).await?;

    Ok(())
}

async fn graphql(query: &str, variables: Value) -> Result<Value, Box<dyn std::error::Error>> {
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
