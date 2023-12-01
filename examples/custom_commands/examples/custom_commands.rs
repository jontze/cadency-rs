#[macro_use]
extern crate log;
#[macro_use]
extern crate cadency_codegen;

use cadency_commands::Fib;
use cadency_core::{
    response::{Response, ResponseBuilder},
    setup_commands, utils, Cadency, CadencyCommand, CadencyError,
};
use serenity::{
    all::Mentionable,
    async_trait,
    client::Context,
    model::application::{CommandDataOptionValue, CommandInteraction},
};

// This is your custom command with the name "hello"
#[derive(CommandBaseline, Default)]
#[description = "Say Hello to a user"]
#[argument(name = "user", description = "The user to great", kind = "User")]
struct Hello {}

#[async_trait]
impl CadencyCommand for Hello {
    // The following code will get executed by the cadency command handler if the command is called
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let user_arg = utils::get_option_value_at_position(command.data.options.as_ref(), 0)
            .and_then(|option_value| {
                if let CommandDataOptionValue::User(user_id) = option_value {
                    Some(user_id)
                } else {
                    error!("Command argument is not a user");
                    None
                }
            })
            .expect("A user as command argument");
        Ok(response_builder
            .message(Some(format!("**Hello {}!**", user_arg.mention())))
            .build()?)
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
    let cadency = Cadency::builder()
        .token("<your_discord_bot_token>".to_string())
        // Add the commands array to cadency
        .commands(commands)
        .build()
        .expect("To build cadency");

    // Start cadency - this will submit and register the commands to discord
    if let Err(why) = cadency.start().await {
        error!("Client error: {:?}", why);
    }
}
