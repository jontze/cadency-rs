use serenity::model::gateway::GatewayIntents;

pub struct CadencyIntents {
    inner: GatewayIntents,
}

impl CadencyIntents {
    pub fn default() -> Self {
        Self {
            inner: GatewayIntents::empty()
                | GatewayIntents::GUILD_VOICE_STATES
                | GatewayIntents::GUILDS,
        }
    }

    pub fn inner(&self) -> GatewayIntents {
        self.inner
    }
}

impl From<CadencyIntents> for GatewayIntents {
    fn from(c_intents: CadencyIntents) -> Self {
        c_intents.inner()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_have_voice_intent() {
        let c_intents: GatewayIntents = CadencyIntents::default().into();
        assert!(c_intents.guild_voice_states());
    }

    #[test]
    fn should_have_guild_intents() {
        let c_intents: GatewayIntents = CadencyIntents::default().into();
        assert!(c_intents.guilds());
    }
}
