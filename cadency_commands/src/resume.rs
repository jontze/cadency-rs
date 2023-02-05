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
        if let Some(guild_id) = command.guild_id {
            let manager = utils::voice::get_songbird(ctx).await;
            if let Some(call) = manager.get(guild_id) {
                let handler = call.lock().await;
                if handler.queue().is_empty() {
                    utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to resume**")
                        .await?;
                } else {
                    match handler.queue().resume() {
                        Ok(_) => {
                            utils::voice::edit_deferred_response(
                                ctx,
                                command,
                                ":play_pause: **Resumed**",
                            )
                            .await?;
                        }
                        Err(err) => {
                            error!("Failed to resume: {err:?}");
                            utils::voice::edit_deferred_response(
                                ctx,
                                command,
                                ":x: **Could not resume**",
                            )
                            .await?;
                        }
                    };
                }
            } else {
                utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to resume**")
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
