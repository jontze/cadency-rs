use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline, Default)]
#[description = "Stop music and clear the track list"]
#[deferred = true]
pub struct Stop {
    options: Vec<CadencyCommandOption>,
}

#[async_trait]
impl CadencyCommand for Stop {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        if let Some(guild_id) = command.guild_id {
            let manager = utils::voice::get_songbird(ctx).await;
            if let Some(call) = manager.get(guild_id) {
                let handler = call.lock().await;
                if handler.queue().is_empty() {
                    utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to stop**")
                        .await?;
                } else {
                    handler.queue().stop();
                    utils::voice::edit_deferred_response(
                        ctx,
                        command,
                        ":white_check_mark: :wastebasket: **Successfully stopped and cleared the playlist**",
                    )
                    .await?;
                }
            } else {
                utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to stop**")
                    .await?;
            }
        } else {
            utils::create_response(
                ctx,
                command,
                ":x: **This command can only be executed on a server**",
            )
            .await?;
        }
        Ok(())
    }
}
