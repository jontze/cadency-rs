use crate::error::CadencyError;
use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        gateway::Activity,
        user::OnlineStatus,
    },
};

#[cfg(feature = "audio")]
pub mod voice;

/// Set the online status and activity of the bot.
/// Should not be set before the `ready` event.
pub async fn set_bot_presence(ctx: &Context) {
    #[cfg(feature = "audio")]
    ctx.set_presence(Some(Activity::listening("music")), OnlineStatus::Invisible)
        .await;
    #[cfg(not(feature = "audio"))]
    ctx.set_presence(Some(Activity::playing("Rust")), OnlineStatus::Invisible)
        .await
}

pub async fn create_response<'a>(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
    content: &str,
) -> Result<(), CadencyError> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(content))
        })
        .await
        .map_err(|err| {
            error!("Failed to submit response: {}", err);
            CadencyError::Response
        })
}

pub async fn create_response_with_embed<'a>(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
    embeds: Vec<CreateEmbed>,
) -> Result<(), CadencyError> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    for embed in embeds {
                        message.add_embed(embed);
                    }
                    message
                })
        })
        .await
        .map_err(|err| {
            error!("Failed to submit embed response: {}", err);
            CadencyError::Response
        })
}
