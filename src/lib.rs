use worker::*;

mod httpw;
mod leetcode;
mod telegram;

const LEETCODE_CLOWN_BOT_TOKEN: &str = "LEETCODE_CLOWN_BOT_TOKEN";
const CHAT_ID: &str = "CHAT_ID";

#[event(fetch)]
async fn fetch(_req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response> {
    let question = leetcode::fetch_question()
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    let bot_token = _env
        .var(LEETCODE_CLOWN_BOT_TOKEN)
        .map_err(|e| worker::Error::RustError(e.to_string()))?
        .to_string();

    let chat_id = _env
        .var(CHAT_ID)
        .map_err(|e| worker::Error::RustError(e.to_string()))?
        .to_string();

    let respnse = telegram::send_message(&bot_token, &chat_id, &question)
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Response::from_json(&respnse)
}
