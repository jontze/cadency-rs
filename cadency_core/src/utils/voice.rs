use crate::{error::CadencyError, http::get_http_client, utils};
use serenity::{
    all::{Guild, GuildId},
    cache::CacheRef,
    client::Context,
    model,
    model::application::CommandInteraction,
};
use songbird::{
    input::{AuxMetadata, Input, YoutubeDl},
    tracks::TrackHandle,
    typemap::TypeMapKey,
    Songbird,
};

pub struct TrackMetaKey;

impl TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

pub fn get_active_voice_channel_id(
    guild: CacheRef<'_, GuildId, Guild>,
    user_id: model::id::UserId,
) -> Option<model::id::ChannelId> {
    guild
        .voice_states
        .get(&user_id)
        .and_then(|voice_state| voice_state.channel_id)
}

pub async fn join(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<
    (
        std::sync::Arc<Songbird>,
        std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
        serenity::model::id::GuildId,
    ),
    CadencyError,
> {
    let manager = get_songbird(ctx).await;
    let guild_id = command.guild_id.ok_or(CadencyError::Command {
        message: ":x: *To use this command, you must be on a server*".to_string(),
    })?;
    let channel_id = ctx
        .cache
        .guild(guild_id)
        .and_then(|guild_cache_ref| {
            utils::voice::get_active_voice_channel_id(guild_cache_ref, command.user.id)
        })
        .ok_or(CadencyError::Join)?;
    debug!("Try to join guild with id: {:?}", guild_id);
    // Skip channel join if already connected
    if let Some(call) = manager.get(guild_id) {
        let has_current_connection = {
            let handler = call.lock().await;
            handler.current_connection().is_some()
        };
        if has_current_connection {
            debug!("Bot is already connected to a channel in the guild.");
            return Ok((manager, call, guild_id));
        }
    }
    // Construct the call for the channel, don't join just yet
    let call = manager.join(guild_id, channel_id).await.map_err(|err| {
        error!("Unable to construct call for channel: {err:?}");
        CadencyError::Join
    })?;
    // Join the channel, this is scoped to drop the lock as soon as possible
    {
        let mut locked_call = call.lock().await;
        locked_call.join(channel_id).await.map_err(|err| {
            error!("Voice channel join failed: {err:?}");
            CadencyError::Join
        })?;
    }
    Ok((manager, call, guild_id))
}

pub async fn add_song(
    context: &Context,
    call: std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    payload: String,
    is_url: bool,
) -> Result<(songbird::input::AuxMetadata, TrackHandle), songbird::input::AuxMetadataError> {
    debug!("Add song to playlist: '{payload}'");
    let request_client = get_http_client(context).await;
    // Create the YoutubeDL source from url or search string
    let source = if is_url {
        YoutubeDl::new(request_client, payload)
    } else {
        YoutubeDl::new(request_client, format!("ytsearch1:{payload}"))
    };
    let mut handler = call.lock().await;

    // Extract metadata and enqueue the source
    let mut input: Input = source.into();
    let metadata = input.aux_metadata().await?;

    let track_handle = handler.enqueue_input(input).await;
    // Store the metadata for later use
    track_handle
        .typemap()
        .write()
        .await
        .insert::<TrackMetaKey>(metadata.clone());

    Ok((metadata, track_handle))
}

pub async fn get_songbird(ctx: &Context) -> std::sync::Arc<songbird::Songbird> {
    songbird::get(ctx)
        .await
        .expect("Failed to get songbird manager")
}
