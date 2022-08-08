#![cfg(feature = "audio")]
use crate::error::CadencyError;
use crate::utils;
use reqwest::Url;
use serenity::model;
use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::application::interaction::{
        application_command::{
            ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
        },
        InteractionResponseType,
    },
};
use songbird::{input::Input, input::Restartable};

pub fn get_active_voice_channel_id(
    guild: model::guild::Guild,
    user_id: model::id::UserId,
) -> Option<model::id::ChannelId> {
    guild
        .voice_states
        .get(&user_id)
        .and_then(|voice_state| voice_state.channel_id)
}

pub async fn join(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<
    (
        std::sync::Arc<songbird::Songbird>,
        serenity::model::id::GuildId,
        serenity::model::id::ChannelId,
    ),
    CadencyError,
> {
    let manager = get_songbird(ctx).await;
    let guild_id = command.guild_id.ok_or(CadencyError::Join)?;
    let channel_id = ctx
        .cache
        .guild(guild_id)
        .and_then(|guild| utils::voice::get_active_voice_channel_id(guild, command.user.id))
        .ok_or(CadencyError::Join)?;
    debug!("Try to join guild with id: {:?}", guild_id);
    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;
        let has_current_connection = handler.current_connection().is_some();
        if has_current_connection {
            debug!("Bot is already connected to a channel in the guild.");
            return Ok((manager, guild_id, channel_id));
        }
    }
    // join the channel
    manager
        .join(guild_id, channel_id)
        .await
        .1
        .map_err(|_err| CadencyError::Join)?;
    Ok((manager, guild_id, channel_id))
}

pub async fn add_song(
    call: std::sync::Arc<tokio::sync::Mutex<songbird::Call>>,
    url: String,
) -> Result<songbird::input::Metadata, songbird::input::error::Error> {
    debug!("Add song to playlist: {}", url);
    let source = Restartable::ytdl(url, true).await?;
    let mut handler = call.lock().await;
    let track: Input = source.into();
    let metadata = *track.metadata.clone();
    handler.enqueue_source(track);
    Ok(metadata)
}

pub fn parse_valid_url(command_options: &[CommandDataOption]) -> Option<reqwest::Url> {
    command_options
        .get(0)
        .and_then(|option| match option.resolved.as_ref() {
            Some(value) => {
                if let CommandDataOptionValue::String(url) = value {
                    Some(url)
                } else {
                    None
                }
            }
            None => None,
        })
        .and_then(|url| Url::parse(url).ok())
}

pub async fn get_songbird(ctx: &Context) -> std::sync::Arc<songbird::Songbird> {
    songbird::get(ctx)
        .await
        .expect("Failed to get songbord manager")
}

pub async fn create_deferred_response<'a>(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
) -> Result<(), CadencyError> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
        })
        .await
        .map_err(|err| {
            error!("Failed to submit deferred message: {err}");
            CadencyError::Response
        })
}

pub async fn edit_deferred_response<'a>(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
    content: &str,
) -> Result<(), CadencyError> {
    interaction
        .edit_original_interaction_response(&ctx.http, |previous_response| {
            previous_response.content(content)
        })
        .await
        .map_err(|err| {
            error!("Failed to edit deferred message: {err}");
            CadencyError::Response
        })?;
    Ok(())
}

pub async fn edit_deferred_response_with_embeded<'a>(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
    embeded_content: CreateEmbed,
) -> Result<(), CadencyError> {
    interaction
        .edit_original_interaction_response(&ctx.http, |previous_response| {
            previous_response.set_embed(embeded_content)
        })
        .await
        .map_err(|err| {
            error!("Failed to edit deferred message with embeded content: {err}");
            CadencyError::Response
        })?;
    Ok(())
}
