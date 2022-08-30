use cadency_core::{utils, CadencyCommand, CadencyError};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command, interaction::application_command::ApplicationCommandInteraction,
    },
};

pub struct Ping;

#[async_trait]
impl CadencyCommand for Ping {
    fn name() -> &'static str {
        "ping"
    }

    /// Construct the slash command that will be submited to the discord api
    async fn register(ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command.name("ping").description("Play Ping-Pong")
            })
            .await?,
        )
    }

    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute ping command");
        utils::create_response(ctx, command, "Pong!").await?;
        Ok(())
    }
}
