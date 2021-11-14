use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::{channel::Message, event::ResumedEvent, gateway::Ready};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _new_message: Message) {
        debug!("{:?}", _new_message);
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        info!("🚀 Start Cadency Discord Bot");
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        debug!("🔌 Reconnect to server");
    }
}
