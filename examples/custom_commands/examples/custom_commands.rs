#[macro_use]
extern crate log;
#[macro_use]
extern crate cadency_codegen;

use cadency_commands::Fib;
use cadency_core::{
    setup_commands, utils, Cadency, CadencyCommand, CadencyCommandOption, CadencyError,
};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

// This is your custom command with the name "hello"
#[derive(CommandBaseline)]
#[description = "Say Hello to a user"]
struct Hello {
    // The allowed list of command arguments
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Hello {
    fn default() -> Self {
        Self {
            options: vec![CadencyCommandOption {
                name: "user",
                description: "The user to greet",
                kind: CommandOptionType::User,
                required: true,
            }],
        }
    }
}

#[async_trait]
impl CadencyCommand for Hello {
    // The following code will get executed by the cadency command handler if the command is called
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        let user_arg = utils::get_option_value_at_position(command.data.options.as_ref(), 0)
            .and_then(|option_value| {
                if let CommandDataOptionValue::User(user, _) = option_value {
                    Some(user)
                } else {
                    error!("Command argument is not a user");
                    None
                }
            })
            .expect("A user as command argument");
        utils::create_response(ctx, command, &format!("**Hello {user_arg}!**",)).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Setup info log level
    let env = env_logger::Env::default().filter_or("RUST_LOG", "cadency=info");
    env_logger::init_from_env(env);

    // Setup an array of all commands for the discord bot
    // The "Fib" command is imported from the cadency commands library.
    // The "Hello" command is your own custom command.
    let commands = setup_commands![Fib::default(), Hello::default()];

    // Init cadency with a valid discord bot token
    let mut cadency = Cadency::new("<your_discord_bot_token>".to_string())
        .await
        .expect("To init cadency")
        // Add the commands array to cadency
        .with_commands(commands)
        .await;

    // Start cadency - this will submit and register the commands to discord
    if let Err(why) = cadency.start().await {
        error!("Client error: {:?}", why);
    }
}
