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
            description: "Play a song from a youtube",
            options: vec![CadencyCommandOption {
                name: "payload",
                description: "Url or search string to the youtube audio source",
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
                    let is_valid_url = Url::parse(string_value).ok().map_or(false, |_| true);
                    Some((string_value, is_valid_url))
                } else {
                    None
                }
            });
        let joined_voice = utils::voice::join(ctx, command).await;
        match (search_data, joined_voice) {
            (Some((search_payload, is_url)), Ok((manager, guild_id, _channel_id))) => {
                let call = manager.get(guild_id).unwrap();
                match utils::voice::add_song(call.clone(), search_payload.clone(), is_url).await {
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
