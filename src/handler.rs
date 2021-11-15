use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        event::ResumedEvent,
        gateway::Ready,
        interactions::{Interaction, InteractionResponseType},
    },
};

use super::commands::setup_commands;

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
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        debug!("🔌 Reconnect to server");
    }
}
