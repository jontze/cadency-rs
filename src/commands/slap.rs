use super::Command;
use serenity::{
    async_trait,
    client::Context,
    model::interactions::{
        application_command::{
            ApplicationCommand, ApplicationCommandInteraction,
            ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
        },
        InteractionResponseType,
    },
};

pub struct Slap;

#[async_trait]
impl Command for Slap {
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
        Ok(
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("slap")
                    .description("Slap someone with a large trout!")
                    .create_option(|option| {
                        option
                            .name("target")
                            .description("The user you want to slap")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
            })
            .await?,
        )
    }

    async fn execute(
        ctx: &Context,
        command: ApplicationCommandInteraction,
    ) -> Result<(), serenity::Error> {
        debug!("Execute slap command");
        let user_option =
            command
                .data
                .options
                .get(0)
                .and_then(|option| match option.resolved.as_ref() {
                    Some(value) => {
                        if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                            value
                        {
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
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.content(format!(
                                        "**Why do you want to slap yourself, {}?**",
                                        command.user
                                    ))
                                })
                        })
                        .await?;
                } else if user.id.0 == command.application_id.0 {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.content(format!(
                                        "**Nope!\n{} slaps {} around a bit with a large trout!**",
                                        user, command.user
                                    ))
                                })
                        })
                        .await?;
                } else {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.content(format!(
                                        "**{} slaps {} around a bit with a large trout!**",
                                        command.user, user
                                    ))
                                })
                        })
                        .await?;
                }
            }
            None => {
                error!("Invalid user input");
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content(":x: *Invalid user provided*")
                            })
                    })
                    .await?;
            }
        };
        Ok(())
    }
}
