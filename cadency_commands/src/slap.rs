use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

#[derive(CommandBaseline)]
pub struct Slap {
    description: &'static str,
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Slap {
    fn default() -> Self {
        Self {
            description: "Slap someone with a large trout!",
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
    #[command]
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
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

        if user.id == command.user.id {
            utils::create_response(
                ctx,
                command,
                &format!("**Why do you want to slap yourself, {}?**", command.user),
            )
            .await?;
        } else if user.id.0 == command.application_id.0 {
            utils::create_response(
                ctx,
                command,
                &format!(
                    "**Nope!\n{} slaps {} around a bit with a large trout!**",
                    user, command.user
                ),
            )
            .await?;
        } else {
            utils::create_response(
                ctx,
                command,
                &format!(
                    "**{} slaps {} around a bit with a large trout!**",
                    command.user, user
                ),
            )
            .await?;
        }
        Ok(())
    }
}
