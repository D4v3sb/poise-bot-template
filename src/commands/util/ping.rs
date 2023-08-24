use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// 🏓 | Check bot ping
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let shard_manager = ctx.framework().shard_manager();

    let shard_manager = shard_manager.lock().await;

    let runners = shard_manager.runners.lock().await;

    let Some(runner) = runners.get(&serenity::ShardId(ctx.serenity_context().shard_id)) else {
            let _ = ctx
                .send(|b| b.content("No shard found").ephemeral(true))
                .await;
            return Ok(());
    };

    let ping = runner.latency.unwrap_or_default();

    ctx.send(|b| {
        b.content(format!("🏓 | Pong. `{:.0?}`", ping))
            .ephemeral(true)
    })
    .await?;
    Ok(())
}
