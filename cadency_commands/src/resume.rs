use cadency_core::{
    response::{Response, ResponseBuilder},
    utils, CadencyCommand, CadencyError,
};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline, Default)]
#[description = "Resume current song if paused"]
#[deferred = true]
pub struct Resume {}

#[async_trait]
impl CadencyCommand for Resume {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let guild_id = command.guild_id.ok_or(CadencyError::Command {
            message: ":x: **This command can only be executed on a server**".to_string(),
        })?;
        let manager = utils::voice::get_songbird(ctx).await;
        let call = manager.get(guild_id).ok_or(CadencyError::Command {
            message: ":x: **No active voice session on the server**".to_string(),
        })?;
        let handler = call.lock().await;
        let response_builder = if handler.queue().is_empty() {
            response_builder.message(Some(":x: **Nothing to resume**".to_string()))
        } else {
            handler.queue().resume().map_err(|err| {
                error!("Failed to resume: {err:?}");
                CadencyError::Command {
                    message: ":x: **Could not resume**".to_string(),
                }
            })?;
            response_builder.message(Some(":play_pause: **Resumed**".to_string()))
        };
        Ok(response_builder.build()?)
    }
}
