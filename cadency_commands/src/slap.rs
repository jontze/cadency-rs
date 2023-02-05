use cadency_core::{
    response::{Response, ResponseBuilder},
    utils, CadencyCommand, CadencyCommandOption, CadencyError,
};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

#[derive(CommandBaseline)]
#[description = "Slap someone with a large trout!"]
pub struct Slap {
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Slap {
    fn default() -> Self {
        Self {
            options: vec![CadencyCommandOption {
                name: "target",
                description: "The user you want to slap",
                kind: CommandOptionType::User,
                required: true,
            }],
        }
    }
}

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
