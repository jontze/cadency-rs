use crate::{command::Commands, CadencyCommand};
use serenity::{
    client::Context,
    model::{
        application::interaction::application_command::{
            CommandDataOption, CommandDataOptionValue,
        },
        gateway::Activity,
        user::OnlineStatus,
    },
};
use std::sync::Arc;

pub mod voice;

/// Set the online status and activity of the bot.
/// Should not be set before the `ready` event.
pub(crate) async fn set_bot_presence(ctx: &Context) {
    ctx.set_presence(Some(Activity::listening("music")), OnlineStatus::Online)
        .await;
}

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
    options
        .get(position)
        .and_then(|option| option.resolved.as_ref())
}
