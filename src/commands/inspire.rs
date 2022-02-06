use super::Command;
use crate::error::CadencyError;
use crate::utils;
use serenity::{
    async_trait,
    client::Context,
    model::interactions::application_command::{ApplicationCommand, ApplicationCommandInteraction},
};

pub struct Inspire;

impl Inspire {
    async fn request_inspire_image_url() -> Result<String, reqwest::Error> {
        debug!("Requesting inspirobot and unpack body");
        Ok(reqwest::get("https://inspirobot.me/api?generate=true")
            .await?
            .text()
            .await?)
    }
}

#[async_trait]
impl Command for Inspire {
    /// Construct the slash command that will be submited to the discord api
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
        Ok(
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("inspire")
                    .description("Say something really inspiring!")
            })
            .await?,
        )
    }

    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute inspire command");
        let inspire_url = Self::request_inspire_image_url().await.map_or_else(
            |err| {
                error!("{:?}", err);
                String::from("The source of my inspiration is currently unavailable.")
            },
            |url| url,
        );
        utils::create_response(ctx, command, &inspire_url).await?;
        Ok(())
    }
}
