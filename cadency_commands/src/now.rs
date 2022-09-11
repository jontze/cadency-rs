use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError, CommandBaseline};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline)]
pub struct Now {
    description: &'static str,
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Now {
    fn default() -> Self {
        Self {
            description: "Show current song",
            options: vec![],
        }
    }
}

#[async_trait]
impl CadencyCommand for Now {
    #[command]
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
                            |title| format!(":newspaper: `{}`", title),
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
