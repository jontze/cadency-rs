use crate::{
    command::{Commands, CommandsScope},
    error::CadencyError,
    handler::command::Handler,
    http::HttpClientKey,
    intents::CadencyIntents,
    CadencyCommand,
};
use serenity::{client::Client, model::gateway::GatewayIntents};
use songbird::SerenityInit;
use std::sync::Arc;

#[derive(derive_builder::Builder)]
pub struct Cadency {
    token: String,
    #[builder(default)]
    commands: Vec<Arc<dyn CadencyCommand>>,
    #[builder(default = "CadencyIntents::default().into()")]
    intents: GatewayIntents,
    /// Used when registering commands with the Discord API
    #[builder(default)]
    commands_scope: CommandsScope,
}

impl Cadency {
    #[must_use]
    pub fn builder() -> CadencyBuilder {
        CadencyBuilder::default()
    }

    /// This will actually start the configured Cadency bot
    pub async fn start(self) -> Result<(), CadencyError> {
        let mut client = Client::builder(self.token, self.intents)
            .event_handler(Handler)
            .register_songbird()
            .type_map_insert::<Commands>(self.commands)
            .type_map_insert::<HttpClientKey>(reqwest::Client::new())
            .type_map_insert::<CommandsScope>(self.commands_scope)
            .await
            .map_err(|err| CadencyError::Start { source: err })?;
        client
            .start()
            .await
            .map_err(|err| CadencyError::Start { source: err })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fail_to_build_without_token() {
        let build = Cadency::builder().commands(vec![]).build();
        assert!(build.is_err())
    }

    #[test]
    fn build_with_token() {
        let build = Cadency::builder()
            .commands(vec![])
            .token("some-token".to_string())
            .build();
        assert!(build.is_ok())
    }

    #[test]
    fn build_with_default_intents() {
        let build = Cadency::builder()
            .commands(vec![])
            .token("some-token".to_string())
            .build()
            .unwrap();
        assert_eq!(build.intents, CadencyIntents::default().into());
    }

    #[test]
    fn build_with_empty_commands() {
        let build = Cadency::builder()
            .commands(vec![])
            .token("some-token".to_string())
            .build()
            .unwrap();
        assert!(build.commands.is_empty());
    }
}
