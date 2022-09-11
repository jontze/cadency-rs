use cadency_core::{utils, CadencyCommand, CadencyError, CommandBaseline};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command, interaction::application_command::ApplicationCommandInteraction,
    },
};

#[derive(CommandBaseline)]
pub struct Skip;

#[async_trait]
impl CadencyCommand for Skip {
    async fn register(&self, ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command.name(self.name()).description("Skip current song")
            })
            .await?,
        )
    }

    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute {} command", self.name());
        if let Some(guild_id) = command.guild_id {
            utils::voice::create_deferred_response(ctx, command).await?;
            let manager = utils::voice::get_songbird(ctx).await;
            if let Some(call) = manager.get(guild_id) {
                let handler = call.lock().await;
                if handler.queue().is_empty() {
                    utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to skip**")
                        .await?;
                } else {
                    match handler.queue().skip() {
                        Ok(_) => {
                            utils::voice::edit_deferred_response(
                                ctx,
                                command,
                                ":fast_forward: **Skipped current song**",
                            )
                            .await?;
                        }
                        Err(err) => {
                            error!("Failed to skip: {err:?}");
                            utils::voice::edit_deferred_response(
                                ctx,
                                command,
                                ":x: **Could not skip**",
                            )
                            .await?;
                        }
                    };
                }
            } else {
                utils::voice::edit_deferred_response(ctx, command, ":x: **Nothing to skip**")
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
