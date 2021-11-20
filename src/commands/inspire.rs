use serenity::{
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
            command
                .name("inspire")
                .description("Say something really inspiring!")
        })
        .await?,
    )
}

pub async fn execute(
    ctx: &Context,
    command: ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    debug!("Execute inspire command");
    let inspire_msg = match reqwest::get("https://inspirobot.me/api?generate=true").await {
        Ok(res) => match res.text().await {
            Ok(inspire_url) => inspire_url,
            Err(err) => {
                error!("Faild parse inspirobot.me response body: {:?}", err);
                String::from("My thoughts are foggy.")
            }
        },
        Err(err) => {
            error!("Failed to load inspirebot.me: {:?}", err);
            String::from("The source of my inspiration is currently unavailable.")
        }
    };
    let inspire_cmd = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(inspire_msg))
        })
        .await?;
    Ok(inspire_cmd)
}
