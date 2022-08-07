use serenity::model::gateway::GatewayIntents;

pub struct CadencyIntents;

impl CadencyIntents {
    pub fn default() -> GatewayIntents {
        GatewayIntents::empty()
    }

    #[cfg(feature = "audio")]
    pub fn with_audio() -> GatewayIntents {
        Self::default() | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS
    }
}
