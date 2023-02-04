use crate::{error::CadencyError, utils};
use serenity::{
    async_trait,
    client::Context,
    model::{
        application::{
            command::Command,
            interaction::{
                application_command::ApplicationCommandInteraction, InteractionResponseType,
            },
        },
        prelude::command::CommandOptionType,
    },
    prelude::TypeMapKey,
};
use std::sync::Arc;

#[macro_export]
macro_rules! setup_commands {
    ($($command_struct:expr),* $(,)*) => {
        {
            let mut commands: Vec<std::sync::Arc<dyn cadency_core::CadencyCommand>> = Vec::new();
            $(
                let command = std::sync::Arc::new($command_struct);
                commands.push(command);
            )*
            commands
        }
    };
}

pub trait CadencyCommandBaseline {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn options(&self) -> &Vec<CadencyCommandOption>;
}

pub struct CadencyCommandOption {
    pub name: &'static str,
    pub description: &'static str,
    pub kind: CommandOptionType,
    pub required: bool,
}

#[async_trait]
pub trait CadencyCommand: Sync + Send + CadencyCommandBaseline {
    /// Construct the slash command that will be submited to the discord api
    async fn register(&self, ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                let command_builder = command.name(self.name()).description(self.description());
                for cadency_option in self.options() {
                    command_builder.create_option(|option_res| {
                        option_res
                            .name(cadency_option.name)
                            .description(cadency_option.description)
                            .kind(cadency_option.kind)
                            .required(cadency_option.required)
                    });
                }
                command_builder
            })
            .await?,
        )
    }
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError>;
}

pub(crate) struct Commands;

impl TypeMapKey for Commands {
    type Value = Vec<Arc<dyn CadencyCommand>>;
}

/// Submit global slash commands to the discord api.
/// As global commands are cached for 1 hour, the activation ca take some time.
/// For local testing it is recommended to create commands with a guild scope.
pub(crate) async fn setup_commands(ctx: &Context) -> Result<(), serenity::Error> {
    let commands = utils::get_commands(ctx).await;
    // No need to run this in parallel as serenity will enforce one-by-one execution
    for command in commands.iter() {
        command.register(ctx).await?;
    }
    Ok(())
}

pub(crate) async fn command_not_implemented(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), CadencyError> {
    error!("The following command is not known: {:?}", command);
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Unknown command"))
        })
        .await
        .map_err(|err| {
            error!("Interaction response failed: {}", err);
            CadencyError::Response
        })
}
