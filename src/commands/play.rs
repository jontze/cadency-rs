#![cfg(feature = "audio")]
use super::Command;
use serenity::{
    async_trait,
    client::Context,
    model::interactions::application_command::{
        ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandOptionType,
    },
};

pub struct Play;

#[async_trait]
impl Command for Play {
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
        Ok(
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("play")
                    .description("Play a song from a youtube url")
                    .create_option(|option| {
                        option
                            .name("url")
                            .description("Url to the youtube audio source")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
            .await?,
        )
    }

    async fn execute(
        _ctx: &Context,
        _command: ApplicationCommandInteraction,
    ) -> Result<(), serenity::Error> {
        debug!("Execute play command");
        todo!();
    }
}
