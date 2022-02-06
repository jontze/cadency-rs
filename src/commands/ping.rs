use super::Command;
use crate::error::CadencyError;
use crate::utils;
use serenity::{
    async_trait,
    client::Context,
    model::interactions::application_command::{ApplicationCommand, ApplicationCommandInteraction},
};

pub struct Ping;

#[async_trait]
impl Command for Ping {
    /// Construct the slash command that will be submited to the discord api
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
        Ok(
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
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
