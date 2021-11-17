use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, event::ResumedEvent, gateway::Ready, interactions::Interaction},
};

use super::client::set_bot_presence;
use super::commands::{command_not_implemented, ping, setup_commands};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _new_message: Message) {
        debug!("{:?}", _new_message);
    }

    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        info!("🚀 Start Cadency Discord Bot");
        match setup_commands(&ctx).await {
            Ok(_) => info!("✅ Application commands submitted"),
            Err(err) => error!("❌ Failed to submit application commands: {:?}", err),
        };
        set_bot_presence(&ctx).await;
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        debug!("🔌 Reconnect to server");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let cmd_execution = match command.data.name.as_str() {
                "ping" => ping::execute(&ctx, command).await,
                _ => command_not_implemented(&ctx, command).await,
            };
            if let Err(execution_err) = cmd_execution {
                error!("❌ Command execution failed: {:?}", execution_err);
            }
        };
    }
}
