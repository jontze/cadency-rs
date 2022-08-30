use cadency_core::{utils, CadencyCommand, CadencyError};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command, interaction::application_command::ApplicationCommandInteraction,
    },
};

pub struct Now;

#[async_trait]
impl CadencyCommand for Now {
    fn name(&self) -> &'static str {
        "now"
    }

    async fn register(&self, ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command.name("now").description("Show current song")
            })
            .await?,
        )
    }

    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute now command");
        if let Some(guild_id) = command.guild_id {
            let manager = songbird::get(ctx).await.expect("Songbord manager");
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
