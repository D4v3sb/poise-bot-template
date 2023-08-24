use poise::serenity_prelude as serenity;

use serenity::{async_trait, EventHandler};

use super::Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, ready: serenity::model::gateway::Ready) {
        if let Some([id, total]) = ready.shard {
            ctx.set_presence(
                Some(serenity::Activity::listening(format!(
                    "Shard {} [{}]",
                    id, total
                ))),
                serenity::OnlineStatus::Online,
            )
            .await;

            info!("Shard {}", id);
        }
    }
}
