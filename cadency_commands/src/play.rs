use cadency_core::{
    handler::voice::InactiveHandler, utils, CadencyCommand, CadencyError, CommandBaseline,
};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::{Command, CommandOptionType},
        interaction::application_command::ApplicationCommandInteraction,
    },
};
use songbird::events::Event;

#[derive(CommandBaseline)]
pub struct Play;

#[async_trait]
impl CadencyCommand for Play {
    async fn register(&self, ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command
                    .name(self.name())
                    .description("Play a song from a youtube url")
                    .create_option(|option| {
                        option
                            .name("url")
                            .description("Url to the youtube audio source")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
            .await?,
        )
    }

    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute {} command", self.name());
        let url_option = utils::voice::parse_valid_url(&command.data.options);
        if let Some(valid_url) = url_option {
            utils::voice::create_deferred_response(ctx, command).await?;
            if let Ok((manager, guild_id, _channel_id)) = utils::voice::join(ctx, command).await {
                let call = manager.get(guild_id).unwrap();
                match utils::voice::add_song(call.clone(), valid_url.to_string()).await {
                    Ok(added_song) => {
                        let mut handler = call.lock().await;
                        handler.remove_all_global_events();
                        handler.add_global_event(
                            Event::Periodic(std::time::Duration::from_secs(120), None),
                            InactiveHandler { guild_id, manager },
                        );
                        utils::voice::edit_deferred_response(
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
                        utils::voice::edit_deferred_response(
                            ctx,
                            command,
                            ":x: **Could not add audio source to the queue!**",
                        )
                        .await?;
                    }
                }
            } else {
                utils::voice::edit_deferred_response(
                    ctx,
                    command,
                    ":x: **Could not join your voice channel**",
                )
                .await?;
            }
        } else {
            utils::create_response(ctx, command, ":x: **This doesn't look lik a valid url**")
                .await?;
        };
        Ok(())
    }
}
