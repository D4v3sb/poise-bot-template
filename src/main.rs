use poise::serenity_prelude as serenity;
use std::env;

#[macro_use]
extern crate log;
mod events;

mod commands;

use commands::util;

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Initalized");

    let version = env::var("CARGO_PKG_VERSION").unwrap_or("Not found".to_string());

    info!("Version: {}", version);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![util::ping::ping()],
            ..Default::default()
        })
        .client_settings(|c| c.event_handler(events::Handler))
        .token(std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(
            |ctx: &serenity::Context,
             _: &serenity::Ready,
             framework: &poise::Framework<Data, _>| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data {})
                })
            },
        );

    framework.run_autosharded().await.unwrap();
}
