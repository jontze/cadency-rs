use cadency_core::{
    handler::voice::InactiveHandler, utils, CadencyCommand, CadencyCommandOption, CadencyError,
};
use reqwest::Url;
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};
use songbird::events::Event;

#[derive(CommandBaseline)]
pub struct Play {
    description: &'static str,
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Play {
    fn default() -> Self {
        Self {
            description: "Play a song from Youtube",
            options: vec![CadencyCommandOption {
                name: "query",
                description: "URL or search query like: 'Africa Toto'",
                kind: CommandOptionType::String,
                required: true,
            }],
        }
    }
}

#[async_trait]
impl CadencyCommand for Play {
    #[command]
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        utils::voice::create_deferred_response(ctx, command).await?;
        let search_data = utils::get_option_value_at_position(command.data.options.as_ref(), 0)
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
            });
        let joined_voice = utils::voice::join(ctx, command).await;
        match (search_data, joined_voice) {
            (Some((search_payload, is_url, is_playlist)), Ok((manager, guild_id, _channel_id))) => {
                let call = manager.get(guild_id).unwrap();
                if is_playlist {
                    let playlist_items =
                        cadency_yt_playlist::fetch_playlist_songs(search_payload.clone()).unwrap();
                    playlist_items
                        .messages
                        .iter()
                        .for_each(|entry| info!("Unable to parse song from playlist: {entry:?}",));
                    let songs = playlist_items.data;
                    let mut amount = 0;
                    let mut total_duration = 0_f32;
                    for song in songs {
                        if amount <= 30 && song.duration <= 600_f32 {
                            match utils::voice::add_song(call.clone(), song.url, true).await {
                                Ok(added_song) => {
                                    amount += 1;
                                    total_duration += song.duration;
                                    debug!("Added song '{:?}' from playlist", added_song.title);
                                }
                                Err(err) => {
                                    error!("Failed to add song: {err}");
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
                    utils::voice::edit_deferred_response(
                        ctx,
                        command,
                        &format!(
                            ":white_check_mark: **Added ___{}___ songs to the queue with a duration of ___{:.2}___ mins** \n**Playing** :notes: `{}`",
                            amount,
                            total_duration,
                            search_payload,
                        ),
                    )
                    .await?;
                } else {
                    match utils::voice::add_song(call.clone(), search_payload.clone(), is_url).await
                    {
                        Ok(added_song) => {
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
                            utils::voice::edit_deferred_response(
                                ctx,
                                command,
                                &format!(
                                    ":white_check_mark: **Added song to the queue** \n**Playing** :notes: `{}` \n:newspaper: `{}`",
                                    song_url,
                                    added_song
                                        .title
                                        .as_ref()
                                        .map_or(":x: **Unknown title**", |title| title)
                                ),
                            )
                            .await?;
                        }
                        Err(err) => {
                            error!("Failed to add song to queue: {}", err);
                            utils::voice::edit_deferred_response(
                                ctx,
                                command,
                                ":x: **Couldn't add audio source to the queue!**",
                            )
                            .await?;
                        }
                    };
                }
            }
            (None, _) => {
                utils::voice::edit_deferred_response(
                    ctx,
                    command,
                    ":x: **Couldn't find a search string**",
                )
                .await?;
            }
            (_, Err(err)) => {
                error!("{err}");
                utils::voice::edit_deferred_response(
                    ctx,
                    command,
                    ":x: **Couldn't join your voice channel**",
                )
                .await?;
            }
        };
        Ok(())
    }
}
