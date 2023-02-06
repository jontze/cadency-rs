use cadency_core::{
    response::{Response, ResponseBuilder},
    utils, CadencyCommand, CadencyError,
};
use serenity::{
    async_trait,
    client::Context,
    model::application::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOptionValue,
    },
};

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
        command: &'a mut ApplicationCommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let user = utils::get_option_value_at_position(command.data.options.as_ref(), 0)
            .and_then(|option_value| {
                if let CommandDataOptionValue::User(user, _) = option_value {
                    Some(user)
                } else {
                    error!("Command option is not a user");
                    None
                }
            })
            .ok_or(CadencyError::Command {
                message: ":x: *Invalid user provided*".to_string(),
            })?;

        let response_builder = if user.id == command.user.id {
            response_builder.message(Some(format!(
                "**Why do you want to slap yourself, {}?**",
                command.user
            )))
        } else if user.id.0 == command.application_id.0 {
            response_builder.message(Some(format!(
                "**Nope!\n{} slaps {} around a bit with a large trout!**",
                user, command.user
            )))
        } else {
            response_builder.message(Some(format!(
                "**{} slaps {} around a bit with a large trout!**",
                command.user, user
            )))
        };
        Ok(response_builder.build()?)
    }
}
