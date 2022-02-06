use super::Command;
use crate::error::CadencyError;
use crate::utils;
use serenity::{
    async_trait,
    client::Context,
    model::interactions::application_command::{
        ApplicationCommand, ApplicationCommandInteraction,
        ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
    },
};

pub struct Fib;

impl Fib {
    fn calc(n: &i64) -> f64 {
        let square_five = 5_f64.sqrt();
        let phi = (1.0 + square_five) / 2.0;
        // FIXME: Type conversion as f64 can lead to loss on large ints, find better way
        let asymp = phi.powf(*n as f64) / square_five;
        asymp.round()
    }
}

#[async_trait]
impl Command for Fib {
    /// Construct the slash command that will be submited to the discord api
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
        Ok(
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("fib")
                    .description("Calculate the nth number in the fibonacci series")
                    .create_option(|option| {
                        option
                            .name("number")
                            .description("The number in the fibonacci series")
                            .kind(ApplicationCommandOptionType::Integer)
                            .required(true)
                    })
            })
            .await?,
        )
    }

    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute fib command");
        let number_option =
            command
                .data
                .options
                .get(0)
                .and_then(|option| match option.resolved.as_ref() {
                    Some(value) => {
                        if let ApplicationCommandInteractionDataOptionValue::Integer(fib_value) =
                            value
                        {
                            Some(fib_value)
                        } else {
                            error!("Fib command option not a integer: {:?}", value);
                            None
                        }
                    }
                    None => {
                        error!("Fib command option empty");
                        None
                    }
                });
        let fib_msg = match number_option {
            Some(number) => Self::calc(number).to_string(),
            None => String::from("Invalid number input!"),
        };
        utils::create_response(ctx, command, &fib_msg).await?;
        Ok(())
    }
}
