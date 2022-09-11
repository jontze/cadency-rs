use cadency_core::{utils, CadencyCommand, CadencyError, CommandBaseline};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command, interaction::application_command::ApplicationCommandInteraction,
    },
};

#[derive(CommandBaseline)]
pub struct Inspire;

impl Inspire {
    async fn request_inspire_image_url() -> Result<String, reqwest::Error> {
        debug!("Requesting inspirobot and unpack body");
        reqwest::get("https://inspirobot.me/api?generate=true")
            .await?
            .text()
            .await
    }
}

#[async_trait]
impl CadencyCommand for Inspire {
    /// Construct the slash command that will be submited to the discord api
    async fn register(&self, ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command
                    .name(self.name())
                    .description("Say something really inspiring!")
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
