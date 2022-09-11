use cadency_core::{utils, CadencyCommand, CadencyError, CommandBaseline};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::{Command, CommandOptionType},
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

#[derive(CommandBaseline)]
pub struct Slap;

#[async_trait]
impl CadencyCommand for Slap {
    async fn register(&self, ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command
                    .name(self.name())
                    .description("Slap someone with a large trout!")
                    .create_option(|option| {
                        option
                            .name("target")
                            .description("The user you want to slap")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
            })
            .await?,
        )
    }

    #[command]
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        let args = command.data.options.clone();
        let user_option = args
            .first()
            .and_then(|option| match option.resolved.as_ref() {
                Some(value) => {
                    if let CommandDataOptionValue::User(user, _member) = value {
                        Some(user)
                    } else {
                        error!("Command option is not a user");
                        None
                    }
                }
                None => {
                    error!("Slap command option empty");
                    None
                }
            });
        match user_option {
            Some(user) => {
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
            }
            None => {
                error!("Invalid user input");
                utils::create_response(ctx, command, ":x: *Invalid user provided*").await?;
            }
        };
        Ok(())
    }
}
