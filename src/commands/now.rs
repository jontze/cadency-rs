#![cfg(feature = "audio")]
use crate::commands::Command;
use crate::error::CadencyError;
use crate::utils;
use serenity::{
    async_trait,
    client::Context,
    model::interactions::application_command::{ApplicationCommand, ApplicationCommandInteraction},
};

pub struct Now;

#[async_trait]
impl Command for Now {
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
        Ok(
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command.name("now").description("Show current song")
            })
            .await?,
        )
    }

    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute now command");
        if let Some(guild_id) = command.guild_id {
            let manager = songbird::get(ctx).await.expect("Songbord manager");
            let call = manager.get(guild_id).unwrap();
            let handler = call.lock().await;
            match handler.queue().current() {
                Some(track) => {
                    utils::create_response(
                        ctx,
                        command,
                        &track.metadata().title.as_ref().map_or(
                            String::from(":x: **Could not add audio source to the queue!**"),
                            |title| format!(":newspaper: `{}`", title),
                        ),
                    )
                    .await?;
                }
                None => {
                    utils::create_response(ctx, command, ":x: **No song is playing**").await?;
                }
            };
        } else {
            utils::create_response(
                ctx,
                command,
                ":x: **This command can only be executed on a server**",
            )
            .await?;
        }
        Ok(())
    }
}
