use crate::{command::Commands, error::CadencyError, CadencyCommand};
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
use std::sync::Arc;

pub mod voice;

/// Set the online status and activity of the bot.
/// Should not be set before the `ready` event.
pub(crate) async fn set_bot_presence(ctx: &Context) {
    ctx.set_presence(Some(Activity::listening("music")), OnlineStatus::Online)
        .await;
}

pub(crate) async fn get_commands(ctx: &Context) -> Vec<Arc<dyn CadencyCommand>> {
    let data_read = ctx.data.read().await;
    data_read
        .get::<Commands>()
        .expect("Command array missing")
        .clone()
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
