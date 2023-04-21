use crate::{error::CadencyError, utils};
use reqwest::Url;
use serenity::{
    client::Context,
    model,
    model::application::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
    },
};
use songbird::{input::Input, input::Restartable, Songbird};

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
        .and_then(|guild| utils::voice::get_active_voice_channel_id(guild, command.user.id))
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
    // join the channel
    let (call, join) = manager.join(guild_id, channel_id).await;
    join.map_err(|err| {
        error!("Voice channel join failed: {err:?}");
        CadencyError::Join
    })?;
    Ok((manager, call, guild_id))
}

pub async fn add_song(
    call: std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    payload: String,
    is_url: bool,
    add_lazy: bool,
) -> Result<songbird::input::Metadata, songbird::input::error::Error> {
    debug!("Add song to playlist: '{payload}'");
    let source = if is_url {
        Restartable::ytdl(payload, add_lazy).await?
    } else {
        Restartable::ytdl_search(payload, add_lazy).await?
    };
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
        .expect("Failed to get songbird manager")
}
