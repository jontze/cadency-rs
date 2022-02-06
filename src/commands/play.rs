#![cfg(feature = "audio")]
use crate::commands::Command;
use crate::error::CadencyError;
use crate::handler::voice::InactiveHandler;
use crate::utils;
use serenity::{
    async_trait,
    client::Context,
    model::interactions::application_command::{
        ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandOptionType,
    },
};
use songbird::events::Event;

pub struct Play;

#[async_trait]
impl Command for Play {
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
        Ok(
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("play")
                    .description("Play a song from a youtube url")
                    .create_option(|option| {
                        option
                            .name("url")
                            .description("Url to the youtube audio source")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
            .await?,
        )
    }

    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute play command");
        let url_option = utils::voice::parse_valid_url(&command.data.options);
        if let Some(valid_url) = url_option {
            if let Ok((manager, guild_id, _channel_id)) = utils::voice::join(ctx, command).await {
                let call = manager.get(guild_id).unwrap();
                match utils::voice::add_song(call.clone(), valid_url.to_string()).await {
                    Ok(added_song) => {
                        let mut handler = call.lock().await;
                        handler.remove_all_global_events();
                        handler.add_global_event(
                            Event::Periodic(std::time::Duration::from_secs(30), None),
                            InactiveHandler { guild_id, manager },
                        );
                        utils::create_response(
                            ctx,
                            command,
                            &format!(
                                ":white_check_mark: **Added song to the queue** \n**Playing** :notes: `{}` \n:newspaper: `{}`",
                                valid_url,
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
                        utils::create_response(
                            ctx,
                            command,
                            ":x: **Could not add audio source to the queue!**",
                        )
                        .await?;
                    }
                }
            } else {
                utils::create_response(ctx, command, ":x: **Could not join your voice channel**")
                    .await?;
            }
        } else {
            utils::create_response(ctx, command, ":x: **This doesn't look lik a valid url**")
                .await?;
        };
        Ok(())
    }
}
