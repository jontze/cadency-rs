use crate::{
    error::CadencyError,
    response::{Response, ResponseBuilder},
    utils,
};
use serenity::{
    async_trait,
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::application::{Command, CommandInteraction, CommandOptionType},
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
    fn deferred(&self) -> bool;
    fn options(&self) -> Vec<CadencyCommandOption>;
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
        let command_options: Vec<CreateCommandOption> = self
            .options()
            .into_iter()
            .map(|option| {
                CreateCommandOption::new(option.kind, option.name, option.description)
                    .required(option.required)
            })
            .collect();
        let command_builder = CreateCommand::new(self.name())
            .description(self.description())
            .set_options(command_options);
        Ok(Command::create_global_command(&ctx.http, command_builder).await?)
    }
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError>;
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
    for command in &commands {
        command.register(ctx).await?;
    }
    Ok(())
}

pub(crate) async fn command_not_implemented(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<(), CadencyError> {
    error!("The following command is not known: {:?}", command);

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Unknown command"),
            ),
        )
        .await
        .map_err(|err| {
            error!("Interaction response failed: {}", err);
            CadencyError::Response
        })
}
