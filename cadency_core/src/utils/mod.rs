use crate::{
    command::{Commands, CommandsScope},
    CadencyCommand,
};
use serenity::client::Context;
use std::sync::Arc;

pub mod voice;

pub(crate) async fn get_commands(ctx: &Context) -> Vec<Arc<dyn CadencyCommand>> {
    let data_read = ctx.data.read().await;
    data_read
        .get::<Commands>()
        .expect("Command array missing")
        .clone()
}

pub(crate) async fn get_commands_scope(ctx: &Context) -> CommandsScope {
    let data_read = ctx.data.read().await;
    *data_read
        .get::<CommandsScope>()
        .expect("Commands scope missing")
}
