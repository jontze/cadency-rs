use cadency_core::{
    handler::voice::InactiveHandler,
    response::{Response, ResponseBuilder},
    utils, CadencyCommand, CadencyError,
};
use reqwest::Url;
use serenity::{
    async_trait,
    client::Context,
    model::application::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOptionValue,
    },
};
use songbird::events::Event;

#[derive(CommandBaseline, Default)]
#[description = "Play a song from Youtube"]
#[deferred = true]
#[argument(
    name = "query",
    description = "URL or search query like: 'Hey Jude Beatles'",
    kind = "String"
)]
pub struct Play {}

#[async_trait]
impl CadencyCommand for Play {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let (search_payload, is_url, is_playlist) =
            utils::get_option_value_at_position(command.data.options.as_ref(), 0)
                .and_then(|option_value| {
                    if let CommandDataOptionValue::String(string_value) = option_value {
                        let (is_valid_url, is_playlist): (bool, bool) = Url::parse(string_value)
                            .ok()
                            .map_or((false, false), |valid_url| {
                                let is_playlist: bool = valid_url
                                    .query_pairs()
                                    .find(|(key, _)| key == "list")
                                    .map_or(false, |_| true);
                                (true, is_playlist)
                            });
                        Some((string_value, is_valid_url, is_playlist))
                    } else {
                        None
                    }
                })
                .ok_or(CadencyError::Command {
                    message: ":x: **No search string provided**".to_string(),
                })?;
        let (manager, guild_id, _channel_id) = utils::voice::join(ctx, command).await?;
        let call = manager.get(guild_id).ok_or(CadencyError::Command {
            message: ":x: **No active voice session on the server**".to_string(),
        })?;
        let mut is_queue_empty = {
            let call_handler = call.lock().await;
            call_handler.queue().is_empty()
        };
        let response_builder = if is_playlist {
            let playlist_items =
                cadency_yt_playlist::fetch_playlist_songs(search_payload.clone()).unwrap();
            playlist_items
                .messages
                .iter()
                .for_each(|entry| debug!("🚧 Unable to parse song from playlist: {entry:?}",));
            let songs = playlist_items.data;
            let mut amount = 0;
            let mut total_duration = 0_f32;
            for song in songs {
                // Add max the first 30 songs of the playlist
                // and only if the duration of the song is below 10mins
                if amount <= 30 && song.duration <= 600_f32 {
                    match utils::voice::add_song(
                        call.clone(),
                        song.url,
                        true,
                        !is_queue_empty, // Don't add first song lazy to the queue
                    )
                    .await
                    {
                        Ok(added_song) => {
                            amount += 1;
                            total_duration += song.duration;
                            is_queue_empty = false;
                            debug!("➕ Added song '{:?}' from playlist", added_song.title);
                        }
                        Err(err) => {
                            error!("❌ Failed to add song: {err}");
                        }
                    }
                }
            }
            total_duration /= 60_f32;
            let mut handler = call.lock().await;
            handler.remove_all_global_events();
            handler.add_global_event(
                Event::Periodic(std::time::Duration::from_secs(120), None),
                InactiveHandler { guild_id, manager },
            );
            drop(handler);
            response_builder.message(Some(format!(
                ":white_check_mark: **Added ___{amount}___ songs to the queue with a duration of ___{total_duration:.2}___ mins** \n**Playing** :notes: `{search_payload}`",
            )))
        } else {
            let added_song = utils::voice::add_song(
                call.clone(),
                search_payload.clone(),
                is_url,
                !is_queue_empty, // Don't add first song lazy to the queue
            )
            .await
            .map_err(|err| {
                error!("❌ Failed to add song to queue: {}", err);
                CadencyError::Command {
                    message: ":x: **Couldn't add audio source to the queue!**".to_string(),
                }
            })?;
            let mut handler = call.lock().await;
            handler.remove_all_global_events();
            handler.add_global_event(
                Event::Periodic(std::time::Duration::from_secs(120), None),
                InactiveHandler { guild_id, manager },
            );
            let song_url = if is_url {
                search_payload
            } else {
                added_song
                    .source_url
                    .as_ref()
                    .map_or("unknown url", |url| url)
            };
            response_builder.message(Some(format!(
                ":white_check_mark: **Added song to the queue and started playing:** \n:notes: `{}` \n:link: `{}`",
                song_url,
                added_song
                    .title
                    .as_ref()
                    .map_or(":x: **Unknown title**", |title| title)
        )))
        };
        Ok(response_builder.build()?)
    }
}
