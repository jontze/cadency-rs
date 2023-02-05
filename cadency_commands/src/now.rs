use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline, Default)]
#[description = "Shows current song"]
pub struct Now {
    options: Vec<CadencyCommandOption>,
}

#[async_trait]
impl CadencyCommand for Now {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        let guild_id = command.guild_id.ok_or(CadencyError::Command {
            message: ":x: **This command can only be executed on a server**".to_string(),
        })?;
        let manager = utils::voice::get_songbird(ctx).await;
        let call = manager.get(guild_id).ok_or(CadencyError::Command {
            message: ":x: **No active voice session on the server**".to_string(),
        })?;
        let handler = call.lock().await;
        let track = handler.queue().current().ok_or(CadencyError::Command {
            message: ":x: **No song is playing**".to_string(),
        })?;
        utils::create_response(
            ctx,
            command,
            &track.metadata().title.as_ref().map_or(
                String::from(":x: **Could not add audio source to the queue!**"),
                |title| format!(":newspaper: `{title}`"),
            ),
        )
        .await?;
        Ok(())
    }
}
