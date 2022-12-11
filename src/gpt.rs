use std::sync::Arc;

use chatgpt::prelude::ChatGPT;
use poise::serenity_prelude::{ClientBuilder, Context, TypeMapKey};

pub struct GptKey;

impl TypeMapKey for GptKey {
    type Value = Arc<GptData>;
}

#[derive(Debug, Clone)]
pub struct GptData {
    pub processing: bool,
    pub gpt: ChatGPT,
}

pub trait GptInit {
    fn register_gpt(self, client: ChatGPT) -> Self;
}

impl GptInit for ClientBuilder {
    fn register_gpt(self, client: ChatGPT) -> Self {
        self.type_map_insert::<GptKey>(Arc::new(GptData {
            processing: false,
            gpt: client,
        }))
    }
}

pub async fn gpt(ctx: &Context) -> Option<Arc<GptData>> {
    let data = ctx.data.read().await;
    data.get::<GptKey>().cloned()
}
