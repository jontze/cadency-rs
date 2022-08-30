use crate::error::CadencyError;
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
};

#[async_trait]
pub trait CadencyCommand {
    fn name() -> &'static str;
    async fn register(ctx: &Context) -> Result<Command, serenity::Error>;
    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError>;
}

/// Submit global slash commands to the discord api.
/// As global commands are cached for 1 hour, the activation ca take some time.
/// For local testing it is recommended to create commands with a guild scope.
pub(crate) async fn setup_commands(ctx: &Context) -> Result<(), serenity::Error> {
    unimplemented!("Register each CadencyCommand");
}

pub(crate) async fn command_not_implemented(
    ctx: &Context,
    command: ApplicationCommandInteraction,
) -> Result<(), CadencyError> {
    error!("The following command is not known: {:?}", command);
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Unknown command"))
        })
        .await
        .map_err(|err| {
            error!("Interaction response failed: {}", err);
            CadencyError::Response
        })
}
