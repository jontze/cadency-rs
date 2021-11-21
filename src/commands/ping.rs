use super::Command;
use serenity::{
    async_trait,
    builder::CreateInteractionResponse,
    client::Context,
    model::interactions::{
        application_command::{ApplicationCommand, ApplicationCommandInteraction},
        InteractionResponseType,
    },
};

pub struct Ping;

impl Ping {
    fn response(response: &mut CreateInteractionResponse) -> &mut CreateInteractionResponse {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Pong!"))
    }
}

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

    async fn execute(
        ctx: &Context,
        command: ApplicationCommandInteraction,
    ) -> Result<(), serenity::Error> {
        debug!("Execute ping command");
        command
            .create_interaction_response(&ctx.http, Self::response)
            .await?;
        Ok(())
    }
}
