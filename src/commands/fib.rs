use super::Command;
use serenity::{
    async_trait,
    builder::CreateInteractionResponse,
    client::Context,
    model::interactions::{
        application_command::{
            ApplicationCommand, ApplicationCommandInteraction,
            ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
        },
        InteractionResponseType,
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

    fn response<'a>(
        response: &'a mut CreateInteractionResponse,
        fib_number: Option<&i64>,
    ) -> &'a mut CreateInteractionResponse {
        let fib_msg = match fib_number {
            Some(number) => Self::calc(number).to_string(),
            None => String::from("Invalid number input!"),
        };
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(fib_msg))
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

    async fn execute(
        ctx: &Context,
        command: ApplicationCommandInteraction,
    ) -> Result<(), serenity::Error> {
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
        command
            .create_interaction_response(&ctx.http, |res| Self::response(res, number_option))
            .await?;
        Ok(())
    }
}
