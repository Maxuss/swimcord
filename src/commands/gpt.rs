use chatgpt::{converse::ChatConversation, prelude::StreamExt, types::ResponsePart};
use poise::{serenity_prelude::Typing, ReplyHandle};

use crate::gpt::gpt;

use super::{CommandResult, Context};

/// Sends a message to ChatGPT and returns it's message
#[tracing::instrument(skip_all)]
#[poise::command(slash_command)]
pub async fn respond(
    ctx: Context<'_>,
    #[description = "Text for ChatGPT to respond to"] text: String,
) -> CommandResult {
    let gpt = gpt(ctx.serenity_context()).await.unwrap();

    let message = ctx
        .send(|reply| reply.content("Please wait, ChatGPT is thinking..."))
        .await?;

    let typing = Typing::start(ctx.serenity_context().http.clone(), ctx.channel_id().0)?;

    let mut stream = gpt.gpt.send_message_streaming(None, None, text).await?;

    let mut count = 0;
    while let Some(item) = stream.next().await {
        let response = item?;
        count += 1;

        if count >= 5 || matches!(response, ResponsePart::Done(_)) {
            count = 0;
            let text = match response {
                ResponsePart::Done(data) => data.message.content.parts[0].to_owned(),
                ResponsePart::Processing(data) => data.message.content.parts[0].to_owned(),
            };
            message.edit(ctx, |reply| reply.content(text)).await?;
        }
    }

    typing.stop().unwrap();

    Ok(())
}

/// Sends a message to a new or existing conversation
#[tracing::instrument(skip_all)]
#[poise::command(slash_command)]
pub async fn converse(
    ctx: Context<'_>,
    #[description = "Text for ChatGPT to respond to"] text: String,
) -> CommandResult {
    let og_data = ctx.data();
    let gpt = gpt(ctx.serenity_context()).await.unwrap();
    let data = og_data.conversation.clone();
    let mut conversation_lock = data.lock().await;
    let conversation = conversation_lock.as_mut();
    let mut conv: ChatConversation;
    let message: ReplyHandle;

    if let Some(data) = conversation {
        message = ctx
            .send(|reply| reply.content("Using previous conversation..."))
            .await?;
        conv = data.clone();
    } else {
        message = ctx
            .send(|reply| reply.content("Initializing new conversation..."))
            .await?;
        conv = gpt.gpt.new_conversation();
    }

    let typing = Typing::start(ctx.serenity_context().http.clone(), ctx.channel_id().0)?;

    let mut stream = conv.send_message_streaming(&gpt.gpt, text).await?;

    let mut count = 0;
    while let Some(item) = stream.next().await {
        let response = item?;
        count += 1;

        if count >= 5 || matches!(response, ResponsePart::Done(_)) {
            count = 0;
            let text = match response {
                ResponsePart::Done(data) => data.message.content.parts[0].to_owned(),
                ResponsePart::Processing(data) => data.message.content.parts[0].to_owned(),
            };
            message.edit(ctx, |reply| reply.content(text)).await?;
        }
    }

    typing.stop().unwrap();

    *conversation_lock = Some(conv);

    Ok(())
}

/// Sends a message to a new or existing conversation
#[tracing::instrument(skip_all)]
#[poise::command(slash_command, ephemeral)]
pub async fn abandon(ctx: Context<'_>) -> CommandResult {
    let og_data = ctx.data();
    *og_data.conversation.lock().await = None;

    ctx.send(|reply| reply.content("Abandoned your existing conversation!"))
        .await?;
    Ok(())
}
