use cadency_core::{
    response::{Response, ResponseBuilder},
    CadencyCommand, CadencyError,
};
use serenity::{
    async_trait, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

#[derive(CommandBaseline, Default)]
#[description = "Play Ping-Pong"]
pub struct Ping {}

#[async_trait]
impl CadencyCommand for Ping {
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        _command: &'a mut ApplicationCommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        Ok(response_builder
            .message(Some("Pong!".to_string()))
            .build()?)
    }
}
