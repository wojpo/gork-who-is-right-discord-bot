use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::error::Error as StdError;

mod commands;
mod utils;

struct Data {}
type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn StdError + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::who_is_right()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;
    Ok(client.start().await?)
}