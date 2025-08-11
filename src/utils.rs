use poise::serenity_prelude as serenity;
use reqwest::Client;
use serde_json::{json, Value};
use crate::{Context, Error};

pub const PROMPT_TEMPLATE: &str = "Na podstawie nastepujacej konwersacji okresl kto ma racje zrob doglebna analize prosimy o odpowiedz w fajnopolackim stylu:\n\n{}";

pub fn build_message_context(messages: &[serenity::model::channel::Message]) -> String {
    let mut message_context = String::new();
    for message in messages.iter().skip(1).rev() {
        message_context.push_str(&format!("{}: {}\n", message.author.name, message.content));
    }
    message_context
}

pub fn create_prompt(message_context: &str) -> String {
    format!("{}", PROMPT_TEMPLATE.replace("{}", message_context))
}

pub async fn call_gemini_api(prompt: &str) -> Result<String, Error> {
    let client = Client::new();
    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("missing GEMINI_API_KEY");

    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent")
        .header("Content-Type", "application/json")
        .header("X-goog-api-key", gemini_api_key)
        .json(&json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": prompt
                        }
                    ]
                }
            ]
        }))
        .send()
        .await?;

    let gemini_response: Value = response.json().await?;
    let answer = gemini_response["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("No response from Gemini API")
        .to_string();

    Ok(answer)
}

pub async fn send_chunked_response(ctx: Context<'_>, answer: &str) -> Result<(), Error> {
    const MAX_MESSAGE_LENGTH: usize = 2000;
    let prefix = "Gemini API response: ";
    let mut remaining = answer;
    let mut first_message = true;

    while !remaining.is_empty() {
        let chunk_size = if first_message {
            MAX_MESSAGE_LENGTH - prefix.len()
        } else {
            MAX_MESSAGE_LENGTH
        };

        let (chunk, rest) = if remaining.len() <= chunk_size {
            (remaining, "")
        } else {
            let mut split_idx = chunk_size;
            while split_idx > 0 && !remaining.is_char_boundary(split_idx) {
                split_idx -= 1;
            }
            while split_idx > 0 && !remaining[..split_idx].ends_with('\n') && !remaining[..split_idx].ends_with(' ') {
                split_idx -= 1;
            }
            if split_idx == 0 {
                split_idx = chunk_size;
            }
            remaining.split_at(split_idx)
        };

        let message = if first_message {
            format!("{}{}", prefix, chunk.trim_end())
        } else {
            chunk.trim_end().to_string()
        };
        ctx.say(message).await?;
        remaining = rest.trim_start();
        first_message = false;
    }

    Ok(())
}