use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline)]
pub struct Inspire {
    description: &'static str,
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Inspire {
    fn default() -> Self {
        Self {
            description: "Say something really inspiring!",
            options: vec![],
        }
    }
}

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
    #[command]
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
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
