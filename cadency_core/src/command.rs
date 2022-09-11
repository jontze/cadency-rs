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

pub trait CommandBaseline {
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
pub trait CadencyCommand: Sync + Send + CommandBaseline {
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
    type Value = std::sync::Arc<Vec<Box<dyn CadencyCommand>>>;
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
    command: ApplicationCommandInteraction,
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

#[cfg(test)]
mod test {
    use super::{CadencyCommandOption, CommandBaseline};

    #[test]
    fn impl_commandbaseline_trait_with_macro() {
        #[derive(cadency_codegen::CommandBaseline)]
        struct Test {
            description: String,
            options: Vec<CadencyCommandOption>,
        }
        assert!(true)
    }

    #[test]
    fn return_lowercase_struct_name_as_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        struct Test {
            description: String,
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            description: "123".to_string(),
            options: Vec::new(),
        };
        let name: String = test.name();
        assert_eq!(name, "test", "Test command name ton be lowercase {name}")
    }

    #[test]
    fn not_return_uppercase_struct_name_as_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        struct Test {
            description: String,
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            description: "123".to_string(),
            options: Vec::new(),
        };
        let name: String = test.name();
        assert_ne!(
            name, "Test",
            "Testing that the first char is not uppercase: {name}"
        )
    }
}
