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
        if let Some(guild_id) = command.guild_id {
            let manager = utils::voice::get_songbird(ctx).await;
            let call = manager.get(guild_id).unwrap();
            let handler = call.lock().await;
            match handler.queue().current() {
                Some(track) => {
                    utils::create_response(
                        ctx,
                        command,
                        &track.metadata().title.as_ref().map_or(
                            String::from(":x: **Could not add audio source to the queue!**"),
                            |title| format!(":newspaper: `{title}`"),
                        ),
                    )
                    .await?;
                }
                None => {
                    utils::create_response(ctx, command, ":x: **No song is playing**").await?;
                }
            };
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
