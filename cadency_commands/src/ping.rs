use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline, Default)]
#[description = "Play Ping-Pong"]
pub struct Ping {
    options: Vec<CadencyCommandOption>,
}

#[async_trait]
impl CadencyCommand for Ping {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        utils::create_response(ctx, command, "Pong!").await?;
        Ok(())
    }
}
