use chatgpt::prelude::ChatGPT;
use commands::{
    gpt::{abandon, converse, respond},
    *,
};
use gpt::GptInit;
use poise::{Framework, FrameworkOptions};

use serenity::prelude::*;
use songbird::SerenityInit;

mod commands;
pub mod err;
pub mod gpt;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let gpt_key =
        std::env::var("SESSION_TOKEN").expect("Expected the GPT session token in the environment");
    let mut gpt = ChatGPT::new(gpt_key).unwrap();
    gpt.refresh_token().await.unwrap();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                setup(),
                join(),
                leave(),
                boom(),
                play(),
                uwu(),
                stop(),
                respond(),
                converse(),
                abandon(),
            ],
            ..Default::default()
        })
        .token(token)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::GUILD_VOICE_STATES)
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(UserData::default()) }))
        .client_settings(|f| f.register_songbird().register_gpt(gpt))
        .build()
        .await
        .unwrap();

    framework.start().await.unwrap();
}
