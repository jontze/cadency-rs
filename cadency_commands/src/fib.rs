use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError, CommandBaseline};
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

#[derive(CommandBaseline)]
pub struct Fib {
    description: &'static str,
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Fib {
    fn default() -> Self {
        Self {
            description: "Calculate the nth number in the fibonacci series",
            options: vec![CadencyCommandOption {
                name: "number",
                description: "The number in the fibonacci series",
                kind: CommandOptionType::Integer,
                required: true,
            }],
        }
    }
}

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
impl CadencyCommand for Fib {
    #[command]
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        let number_option =
            command
                .data
                .options
                .get(0)
                .and_then(|option| match option.resolved.as_ref() {
                    Some(value) => {
                        if let CommandDataOptionValue::Integer(fib_value) = value {
                            Some(fib_value)
                        } else {
                            error!("{} command option not a integer: {:?}", self.name(), value);
                            None
                        }
                    }
                    None => {
                        error!("{} command option empty", self.name());
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
