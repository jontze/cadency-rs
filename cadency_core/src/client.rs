use crate::error::CadencyError;
use crate::handler::command::Handler;
use crate::intents::CadencyIntents;
use serenity::client::Client;
use songbird::SerenityInit;

pub const DISCORD_TOKEN_ENV: &str = "DISCORD_TOKEN";

pub struct Cadency {
    client: serenity::Client,
}

impl Cadency {
    /// Construct the Cadency discord but with default configuration
    pub async fn default() -> Result<Self, CadencyError> {
        let token = std::env::var(DISCORD_TOKEN_ENV)
            .map_err(|_| CadencyError::Environment(DISCORD_TOKEN_ENV.to_string()))?;
        Self::new(token).await
    }

    /// Construct the Cadency discord bot with a custom token that can be set programmatically
    ///
    /// # Arguments
    /// * `token` - The discord bot token as string
    pub async fn new(token: String) -> Result<Self, CadencyError> {
        let client = Client::builder(token, CadencyIntents::default().into())
            .event_handler(Handler)
            .register_songbird()
            .await
            .map_err(|err| CadencyError::Builder { source: err })?;
        Ok(Self { client })
    }

    /// This will actually start the configured Cadency bot
    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client.start().await
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn should_error_on_missing_env() {
        env::remove_var(DISCORD_TOKEN_ENV);
        let cadency = Cadency::default().await;
        assert!(cadency.is_err())
    }

    // #[tokio::test]
    // async fn should_build_cadency() {
    //     env::set_var(DISCORD_TOKEN_ENV, "example_token");
    //     let test = Cadency::default().await;
    //     assert!(test.is_ok())
    // }
}
