use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline, Default)]
#[description = "Resume current song if paused"]
#[deferred = true]
pub struct Resume {
    options: Vec<CadencyCommandOption>,
}

#[async_trait]
impl CadencyCommand for Resume {
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
        if handler.queue().is_empty() {
            utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to resume**").await?;
        } else {
            handler.queue().resume().map_err(|err| {
                error!("Failed to resume: {err:?}");
                CadencyError::Command {
                    message: ":x: **Could not resume**".to_string(),
                }
            })?;
            utils::voice::edit_deferred_response(ctx, command, ":play_pause: **Resumed**").await?;
        }
        Ok(())
    }
}
