use cadency_core::{
    response::{Response, ResponseBuilder},
    utils, CadencyCommand, CadencyError,
};
use serenity::{
    async_trait,
    client::Context,
    model::application::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOptionValue,
    },
};

#[derive(CommandBaseline, Default)]
#[description = "Calculate the nth number in the fibonacci sequence"]
#[argument(
    name = "number",
    description = "The number in the fibonacci sequence",
    kind = "Integer"
)]
pub struct Fib {}

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
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let number = utils::get_option_value_at_position(command.data.options.as_ref(), 0)
            .and_then(|option_value| {
                if let CommandDataOptionValue::Integer(fib_value) = option_value {
                    Some(fib_value)
                } else {
                    error!(
                        "{} command option not a integer: {:?}",
                        self.name(),
                        option_value
                    );
                    None
                }
            })
            .ok_or(CadencyError::Command {
                message: "Invalid number input".to_string(),
            })?;
        let fib_msg = Self::calc(number).to_string();
        Ok(response_builder.message(Some(fib_msg)).build()?)
    }
}
