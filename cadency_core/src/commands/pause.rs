use crate::commands::CadencyCommand;
use crate::error::CadencyError;
use crate::utils;
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command, interaction::application_command::ApplicationCommandInteraction,
    },
};

pub struct Pause;

#[async_trait]
impl CadencyCommand for Pause {
    fn name() -> &'static str {
        "pause"
    }

    async fn register(ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command.name("pause").description("Pause current song")
            })
            .await?,
        )
    }

    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute pause command");
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
