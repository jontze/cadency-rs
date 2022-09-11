use cadency_core::{utils, CadencyCommand, CadencyError, CommandBaseline};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command, interaction::application_command::ApplicationCommandInteraction,
    },
};

#[derive(CommandBaseline)]
pub struct Ping;

#[async_trait]
impl CadencyCommand for Ping {
    /// Construct the slash command that will be submited to the discord api
    async fn register(&self, ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command.name(self.name()).description("Play Ping-Pong")
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
        utils::create_response(ctx, command, "Pong!").await?;
        Ok(())
    }
}
