use crate::{Context, Error};
use serenity::all::GetMessages;

///Deletes (up to) the most recent 100 messages
#[poise::command(prefix_command, track_edits, discard_spare_arguments)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "Number of messages"]
    #[max = 99]
    #[min = 1]
    num: Option<u8>,
) -> Result<(), Error> {
    if num.is_none() {
        ctx.say("Need to specify number of messages to delete")
            .await?;
        return Ok(());
    } else if num.unwrap_or(0) > 99 {
        ctx.say("Too many messages; Discord API only allows fetching 100 messages at a time (99 including the purge command).").await?;
        return Ok(());
    } else if num.unwrap_or(2) < 1 {
        ctx.say("Um.. what?").await?;
        return Ok(());
    }
    let cid = ctx.channel_id();
    let to_delete = num.unwrap() + 1;
    let msgs = cid
        .messages(&ctx, GetMessages::new().limit(to_delete))
        .await?;

    cid.delete_messages(&ctx.http(), msgs).await?;

    Ok(())
}
