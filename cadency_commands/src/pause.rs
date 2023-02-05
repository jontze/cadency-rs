use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline, Default)]
#[description = "Pause the current song"]
#[deferred = true]
pub struct Pause {
    options: Vec<CadencyCommandOption>,
}

#[async_trait]
impl CadencyCommand for Pause {
    #[command]
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        if let Some(guild_id) = command.guild_id {
            utils::voice::create_deferred_response(ctx, command).await?;
            let manager = utils::voice::get_songbird(ctx).await;
            if let Some(call) = manager.get(guild_id) {
                let handler = call.lock().await;
                if handler.queue().is_empty() {
                    utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to pause**")
                        .await?;
                } else {
                    match handler.queue().pause() {
                        Ok(_) => {
                            utils::voice::edit_deferred_response(
                                ctx,
                                command,
                                ":pause_button: **Paused**",
                            )
                            .await?;
                        }
                        Err(err) => {
                            error!("Failed to pause: {err:?}");
                            utils::voice::edit_deferred_response(
                                ctx,
                                command,
                                ":x: **Could not pause**",
                            )
                            .await?;
                        }
                    };
                }
            } else {
                utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to pause**")
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
