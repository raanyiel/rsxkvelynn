use poise::serenity_prelude as serenity;

use crate::{Context, Error};

///Get URL to user's avatar!
#[poise::command(slash_command, prefix_command, track_edits, discard_spare_arguments)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    if user.is_none() {
        ctx.say("Need to specify a single user!").await?;
        return Ok(());
    }

    let u = user.as_ref().unwrap();
    ctx.say(u.avatar_url().unwrap_or("Couldn't find URL :(".to_string()))
        .await?;

    Ok(())
}
