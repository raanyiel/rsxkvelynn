use crate::{Context, Error};
use poise::serenity_prelude::all::GetMessages;

///Deletes (up to) the most recent 100 messages
#[poise::command(prefix_command, track_edits, discard_spare_arguments)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "Number of messages"]
    #[rename = "1-99"]
    num: Option<u8>,
) -> Result<(), Error> {
    let to_delete = match num {
        Some(x) => match x {
            100.. => {
                ctx.say("Too many messages; Discord API only allows fetching 100 messages at a time (99 including the purge command).").await?;
                return Ok(());
            }
            ..1 => {
                ctx.say("Um.. what?").await?;
                return Ok(());
            }
            1..=99 => x + 1,
        },
        None => {
            ctx.say("Need to specify number of messages to delete")
                .await?;
            return Ok(());
        }
    };

    let cid = ctx.channel_id();
    let msgs = cid
        .messages(ctx, GetMessages::new().limit(to_delete))
        .await?;

    cid.delete_messages(ctx, msgs).await?;

    Ok(())
}
