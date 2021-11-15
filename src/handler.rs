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

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // TODO: Improve structure for better scale with multiple commands
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Pong!".to_string(),
                _ => "Not implemented :(".to_string(),
            };
            if let Err(response_err) = command
                .create_interaction_response(ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("❌ Interaction response failed: {:?}", response_err);
            }
        };
    }
}
