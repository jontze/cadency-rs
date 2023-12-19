use cadency_core::{
    response::{Response, ResponseBuilder},
    CadencyCommand, CadencyError,
};
use rand::Rng;
use serenity::{async_trait, client::Context, model::application::CommandInteraction};

#[derive(CommandBaseline, Default)]
#[description = "Roll a dice of n sides"]
#[argument(
    name = "sides",
    description = "The number of sides on the dice",
    kind = "Integer"
)]
pub struct Roll {}

impl Roll {
    fn roll_dice(&self, sides: &i64) -> i64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=*sides) as i64
    }
}

#[async_trait]
impl CadencyCommand for Roll {
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let roll = self.roll_dice(&self.arg_sides(command));
        let roll_msg = format!("**:dice_cube: You rolled a `{roll}`**");
        Ok(response_builder.message(Some(roll_msg)).build()?)
    }
}
