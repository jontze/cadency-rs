use serenity::{
    builder::CreateInteractionResponse,
    client::Context,
    model::interactions::{
        application_command::{ApplicationCommand, ApplicationCommandInteraction},
        InteractionResponseType,
    },
};

/// Construct the slash command that will be submited to the discord api
pub async fn create(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
    Ok(
        ApplicationCommand::create_global_application_command(&ctx.http, |command| {
            command.name("ping").description("Play Ping-Pong")
        })
        .await?,
    )
}

pub async fn execute(
    ctx: &Context,
    command: ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    debug!("Execute ping command");
    let ping_cmd = command
        .create_interaction_response(&ctx.http, response)
        .await?;
    Ok(ping_cmd)
}

fn response(response: &mut CreateInteractionResponse) -> &mut CreateInteractionResponse {
    response
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|message| message.content("Pong!"))
}
