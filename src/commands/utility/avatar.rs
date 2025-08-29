use poise::serenity_prelude as serenity;

use crate::{Context, Error};

///Get URL to user's avatar!
#[poise::command(slash_command, prefix_command, track_edits, discard_spare_arguments)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = match user.as_ref() {
        Some(x) => x,
        None => {
            ctx.say("Need to specify a single user!").await?;
            return Ok(());
        }
    };

    ctx.say(
        user.avatar_url()
            .unwrap_or("Couldn't find URL :(".to_string()),
    )
    .await?;

    Ok(())
}
