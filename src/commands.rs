use tracing::info;

use crate::err::CommandError;

type Error = CommandError;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Data;
type CommandResult = Result<(), Error>;

/// Sets up the bot
#[poise::command(prefix_command, owners_only, ephemeral)]
pub async fn setup(ctx: Context<'_>) -> CommandResult {
    poise::builtins::register_application_commands(ctx, false).await?;

    ctx.send(|reply| reply.content("Setup successful").ephemeral(true))
        .await?;
    Ok(())
}

#[inline]
async fn playsound(ctx: Context<'_>, sound: &str, symbol: &str) -> CommandResult {
    let guild = ctx.guild_id().unwrap();
    let mgr = songbird::get(ctx.discord()).await.unwrap();

    if let Some(handler_lock) = mgr.get(guild) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ffmpeg(sound).await {
            Ok(source) => source,
            Err(why) => {
                tracing::error!("Err starting source: {:?}", why);

                ctx.send(|reply| reply.content("Failed to load audio!").ephemeral(true))
                    .await?;

                return Ok(());
            }
        };

        handler.play_source(source);

        ctx.say(symbol).await?;
    } else {
        ctx.send(|reply| reply.content("Not in a voice channel!").ephemeral(true))
            .await?;
    }

    Ok(())
}

/// 🛑 Stops the current playing audio
#[tracing::instrument(skip_all)]
#[poise::command(slash_command)]
pub async fn stop(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.guild_id().unwrap();
    let mgr = songbird::get(ctx.discord()).await.unwrap();
    if let Some(handler_lock) = mgr.get(guild) {
        let mut handler = handler_lock.lock().await;

        handler.stop();

        ctx.say("Stopped current playing audio!").await?;
    } else {
        ctx.send(|reply| reply.content("Not in a voice channel!").ephemeral(true))
            .await?;
    }
    Ok(())
}

/// 🎵 Plays the audio from the provided link
#[tracing::instrument(skip_all)]
#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "URL to be played"] link: String,
) -> CommandResult {
    let guild = ctx.guild_id().unwrap();
    let mgr = songbird::get(ctx.discord()).await.unwrap();

    if let Some(handler_lock) = mgr.get(guild) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&link).await {
            Ok(source) => source,
            Err(why) => {
                tracing::error!("Err starting source: {:?}", why);

                ctx.send(|reply| reply.content("Failed to load audio!").ephemeral(true))
                    .await?;

                return Ok(());
            }
        };

        handler.play_source(source);

        ctx.say(format!("Playing `{link}`")).await?;
    } else {
        ctx.send(|reply| reply.content("Not in a voice channel!").ephemeral(true))
            .await?;
    }

    Ok(())
}

/// 💥 Makes the vine boom sound
#[tracing::instrument(skip_all)]
#[poise::command(slash_command)]
pub async fn boom(ctx: Context<'_>) -> CommandResult {
    playsound(ctx, "./sounds/vineboom.mp3", "💥").await
}

/// 👯‍♂️ Makes the uwu sound
#[tracing::instrument(skip_all)]
#[poise::command(slash_command)]
pub async fn uwu(ctx: Context<'_>) -> CommandResult {
    playsound(ctx, "./sounds/uwu.mp3", "uwu").await
}

/// 🚪 Leaves the current voice channel
#[poise::command(slash_command, ephemeral)]
pub async fn leave(ctx: Context<'_>) -> CommandResult {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if (manager.remove(guild_id).await).is_err() {
            ctx.say("Failed to leave. I am here forever now.").await?;
        }

        ctx.say("Left voice channel").await?;
    } else {
        ctx.say("Not in a voice channel right now").await?;
    }

    Ok(())
}

/// 🚪 /Makes the bot join your channel
#[tracing::instrument(skip_all)]
#[poise::command(slash_command, ephemeral)]
pub async fn join(ctx: Context<'_>) -> CommandResult {
    info!("Joining!");

    let user = ctx.author().id;
    let guild = ctx.guild().ok_or(CommandError::Unknown)?;
    if let Some(voice_state) = guild.voice_states.get(&user) {
        let channel = guild
            .channels
            .get(&voice_state.channel_id.ok_or(CommandError::Unknown)?)
            .unwrap();

        let ctx_discord = ctx.discord();
        let guild_id = ctx.guild_id().unwrap();

        let songbird = songbird::get(ctx_discord).await.unwrap();

        let _handle = songbird.join(guild_id, channel.id()).await;

        ctx.send(|reply| reply.content("Joined your channel"))
            .await?;
    } else {
        ctx.send(|reply| reply.content("You are not in a voice channel!"))
            .await?;
    }

    Ok(())
}
