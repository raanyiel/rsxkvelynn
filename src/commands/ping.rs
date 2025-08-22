use crate::{Context, Error};

///Ping pong!
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("new pong!").await?;
    Ok(())
}
