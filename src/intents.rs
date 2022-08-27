use serenity::model::gateway::GatewayIntents;

pub struct CadencyIntents;

impl CadencyIntents {
    pub fn default() -> GatewayIntents {
        GatewayIntents::empty() | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS
    }
}
