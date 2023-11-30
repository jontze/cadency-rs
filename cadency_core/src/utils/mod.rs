use crate::{command::Commands, CadencyCommand};
use serenity::{
    client::Context,
    model::application::{CommandDataOption, CommandDataOptionValue},
};
use std::sync::Arc;

pub mod voice;

pub(crate) async fn get_commands(ctx: &Context) -> Vec<Arc<dyn CadencyCommand>> {
    let data_read = ctx.data.read().await;
    data_read
        .get::<Commands>()
        .expect("Command array missing")
        .clone()
}

pub fn get_option_value_at_position(
    options: &[CommandDataOption],
    position: usize,
) -> Option<&CommandDataOptionValue> {
    options.get(position).map(|option| &option.value)
}
