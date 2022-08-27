use serenity::async_trait;
use songbird::events::{Event, EventContext};

pub struct InactiveHandler {
    pub manager: std::sync::Arc<songbird::Songbird>,
    pub guild_id: serenity::model::id::GuildId,
}

#[async_trait]
impl songbird::EventHandler for InactiveHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let songbird::EventContext::Track(track_list) = ctx {
            if track_list.is_empty() {
                debug!("Tracklist empty, leave channel!");
                if let Some(call) = self.manager.get(self.guild_id) {
                    let mut handler = call.lock().await;
                    handler.leave().await.unwrap();
                }
            }
        }
        None
    }
}
