use crate::{Context, Error};

/// Help menu
#[poise::command(prefix_command, track_edits, discard_spare_arguments)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "whatcha wanna know about"] _command: Option<String>,
) -> Result<(), Error> {
    let _config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
        if you're somehow seeing this.. you weren't supposed to.",
        ..Default::default()
    };
    ctx.say("not yet.").await?;
    Ok(())
}
