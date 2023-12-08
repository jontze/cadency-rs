use cadency_core::{
    response::{Response, ResponseBuilder},
    CadencyCommand, CadencyError,
};
use serenity::{
    all::Mentionable, async_trait, client::Context, model::application::CommandInteraction,
};
use std::num::NonZeroU64;

#[derive(CommandBaseline, Default)]
#[description = "Slap someone with a large trout!"]
#[argument(
    name = "target",
    description = "The user you want to slap",
    kind = "User"
)]
pub struct Slap {}

#[async_trait]
impl CadencyCommand for Slap {
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let user_id = self.arg_target(command);

        let response_builder = if user_id == command.user.id {
            response_builder.message(Some(format!(
                "**Why do you want to slap yourself, {}?**",
                command.user.mention()
            )))
        } else if NonZeroU64::from(user_id) == NonZeroU64::from(command.application_id) {
            response_builder.message(Some(format!(
                "**Nope!\n{} slaps {} around a bit with a large trout!**",
                user_id.mention(),
                command.user.mention()
            )))
        } else {
            response_builder.message(Some(format!(
                "**{} slaps {} around a bit with a large trout!**",
                command.user.mention(),
                user_id.mention()
            )))
        };
        Ok(response_builder.build()?)
    }
}
