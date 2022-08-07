use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{application::interaction::Interaction, event::ResumedEvent, gateway::Ready},
};

use crate::commands::{
    command_not_implemented, setup_commands, CadencyCommand, Fib, Inspire, Ping, Slap, Urban,
};
#[cfg(feature = "audio")]
use crate::commands::{Now, Play, Skip};
use crate::utils::set_bot_presence;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        info!("🚀 Start Cadency Discord Bot");
        set_bot_presence(&ctx).await;
        match setup_commands(&ctx).await {
            Ok(_) => info!("✅ Application commands submitted"),
            Err(err) => error!("❌ Failed to submit application commands: {:?}", err),
        };
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        debug!("🔌 Reconnect to server");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            let cmd_execution = match command.data.name.as_str() {
                "ping" => Ping::execute(&ctx, &mut command).await,
                "inspire" => Inspire::execute(&ctx, &mut command).await,
                "fib" => Fib::execute(&ctx, &mut command).await,
                "urban" => Urban::execute(&ctx, &mut command).await,
                "slap" => Slap::execute(&ctx, &mut command).await,
                #[cfg(feature = "audio")]
                "play" => Play::execute(&ctx, &mut command).await,
                #[cfg(feature = "audio")]
                "now" => Now::execute(&ctx, &mut command).await,
                #[cfg(feature = "audio")]
                "skip" => Skip::execute(&ctx, &mut command).await,
                _ => command_not_implemented(&ctx, command).await,
            };
            if let Err(execution_err) = cmd_execution {
                error!("❌ Command execution failed: {:?}", execution_err);
            }
        };
    }
}
