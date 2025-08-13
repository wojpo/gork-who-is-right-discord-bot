use poise::serenity_prelude as serenity;
use crate::{Context, Error, utils};

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn who_is_right(
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

    ctx.defer().await?;

    let message_context = utils::build_message_context(&messages);
    let prompt = utils::create_prompt(&message_context);
    let answer = utils::call_gemini_api(&prompt).await?;

    utils::send_chunked_response(ctx, &answer).await?;

    Ok(())
}