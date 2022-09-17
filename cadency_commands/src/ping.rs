use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline)]
pub struct Ping {
    description: &'static str,
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Ping {
    fn default() -> Self {
        Self {
            description: "Play Ping-Pong",
            options: vec![],
        }
    }
}

#[async_trait]
impl CadencyCommand for Ping {
    #[command]
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        utils::create_response(ctx, command, "Pong!").await?;
        Ok(())
    }
}
