use cadency_core::{
    response::{Response, ResponseBuilder},
    CadencyCommand, CadencyError,
};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline, Default)]
#[description = "Say something really inspiring!"]
pub struct Inspire {}

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
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        _command: &'a mut ApplicationCommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let inspire_url = Self::request_inspire_image_url().await.map_err(|err| {
            error!("{:?}", err);
            CadencyError::Command {
                message: "**The source of my inspiration is currently unavailable :(**".to_string(),
            }
        })?;
        Ok(response_builder.message(Some(inspire_url)).build()?)
    }
}
