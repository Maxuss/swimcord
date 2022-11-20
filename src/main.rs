use commands::*;
use poise::{Framework, FrameworkOptions};

use serenity::prelude::*;
use songbird::SerenityInit;

mod commands;
pub mod err;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![setup(), join(), leave(), boom()],
            ..Default::default()
        })
        .token(token)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::GUILD_VOICE_STATES)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data) }))
        .client_settings(|f| f.register_songbird())
        .build()
        .await
        .unwrap();

    framework.start().await.unwrap();
}
