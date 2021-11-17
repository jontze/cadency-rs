use serenity::{
    client::Context,
    model::interactions::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

mod ping;

/// Submit global slash commands to the discord api.
/// As global commands are cached for 1 hour, the activation ca take some time.
/// For local testing it is recommended to create commandswith a guild scope.
pub async fn setup_commands(ctx: &Context) -> Result<(), serenity::Error> {
    let _ping_cmd = ping::create(ctx).await?;
    Ok(())
}

pub async fn command_not_implemented(
    ctx: &Context,
    command: ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    error!("The following command is not known: {:?}", command);
    let unknown_command = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Unknown command"))
        })
        .await?;
    Ok(unknown_command)
}
