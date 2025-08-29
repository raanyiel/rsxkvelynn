use core::fmt::{Debug, Display};
use std::sync::Arc;

use poise::{FrameworkError, serenity_prelude as serenity};

mod commands;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn c_on_error<U, E: Display + Debug>(error: FrameworkError<'_, U, E>) {
    match error {
        FrameworkError::ArgumentParse { error, ctx, .. } => {
            let _ = ctx.say(format!("DEV NOTE: {}", error.to_string())).await;
        }
        _ => {
            if let Err(e) = poise::builtins::on_error(error).await {
                eprintln!("Fatal error while sending error message: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("env variable DISCORD_TOKEN missing");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                ))),
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: vec![
                commands::utility::ping::ping(),
                commands::utility::help::help(),
                commands::utility::avatar::avatar(),
                commands::utility::purge::purge(),
            ],
            on_error: |error| Box::pin(c_on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
