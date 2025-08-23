use crate::{Context, Error};

///Ping pong!
#[poise::command(slash_command, prefix_command, track_edits, discard_spare_arguments)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("new pong!").await?;
    Ok(())
}
