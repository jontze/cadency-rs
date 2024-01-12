use super::dice::{RollDice, Throw};
use cadency_core::{
    response::{Response, ResponseBuilder},
    CadencyCommand, CadencyError,
};
use serenity::{async_trait, client::Context, model::application::CommandInteraction};

#[derive(CommandBaseline, Default)]
#[description = "Roll a dice of n sides"]
#[argument(
    name = "roll",
    description = "Dice(s) to roll. Only the following patterns are supported: `d6`, `2d6`, 2d6+1` or `2d6-1`",
    kind = "String"
)]
pub struct Roll {}

impl Roll {}

#[async_trait]
impl CadencyCommand for Roll {
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let throw_str = self.arg_roll(command);
        let throw = throw_str.parse::<Throw>()?;

        throw.validate()?;

        let roll = throw.roll();

        let roll_msg = format!("**`{throw_str}` :ice_cube: You rolled a `{roll}`**");
        Ok(response_builder.message(Some(roll_msg)).build()?)
    }
}
