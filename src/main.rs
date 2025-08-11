use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn who_is_right(
    ctx: Context<'_>,
    #[description = "Number of messages to fetch (max 100)"]
    count: Option<u64>,
) -> Result<(), Error> {
    let count = count.unwrap_or(49) + 1;
    let count = count.min(100);

    let messages = ctx
        .channel_id()
        .messages(
            &ctx,
            serenity::builder::GetMessages::new().limit(count.try_into().unwrap()),
        )
        .await?;

    let mut message_context = String::new();
    for message in messages.iter().skip(1).rev() {
        message_context.push_str(&format!("{}: {}\n", message.author.name, message.content));
    }

    let client = Client::new();
    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("missing GEMINI_API_KEY");
    let prompt = format!("Based on the following conversation context, who is right?\n\n{}", message_context);

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

    const MAX_MESSAGE_LENGTH: usize = 2000;
    let prefix = "Gemini API response: ";
    let mut remaining = answer.as_str();
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
        ctx.reply(message).await?;
        remaining = rest.trim_start();
        first_message = false;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![who_is_right()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}