use crate::{
    command::Commands, error::CadencyError, handler::command::Handler, intents::CadencyIntents,
    CadencyCommand,
};
use serenity::client::Client;
use songbird::SerenityInit;
#[cfg(not(test))]
use std::env;
use std::sync::Arc;

#[cfg_attr(test, mockall::automock)]
mod env_read {
    #[allow(dead_code)]
    pub fn var(_env_name: &str) -> Result<String, ()> {
        Ok(String::from("SOME_ENV_VALUE"))
    }
}

#[cfg(test)]
use mock_env_read as env;

pub const DISCORD_TOKEN_ENV: &str = "DISCORD_TOKEN";

pub struct Cadency {
    client: serenity::Client,
}

#[cfg_attr(test, mockall::automock)]
impl Cadency {
    /// Construct the Cadency discord but with default configuration
    pub async fn default() -> Result<Self, CadencyError> {
        let token = env::var(DISCORD_TOKEN_ENV)
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

    /// This will register and provide the commands for cadency.
    /// Every struct that implements the CadencyCommand trait can be used.
    pub async fn with_commands(self, commands: Vec<Arc<dyn CadencyCommand>>) -> Self {
        {
            let mut data = self.client.data.write().await;
            data.insert::<Commands>(commands);
        }
        self
    }

    /// This will actually start the configured Cadency bot
    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client.start().await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static MTX: Mutex<()> = Mutex::new(());

    // When a test panics, it will poison the Mutex. Since we don't actually
    // care about the state of the data we ignore that it is poisoned and grab
    // the lock regardless.  If you just do `let _m = &MTX.lock().unwrap()`, one
    // test panicking will cause all other tests that try and acquire a lock on
    // that Mutex to also panic.
    fn get_lock(m: &'static Mutex<()>) -> MutexGuard<'static, ()> {
        match m.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[tokio::test]
    async fn should_error_on_missing_env_token() {
        let _m = get_lock(&MTX);

        let env_cxt = mock_env_read::var_context();
        env_cxt.expect().return_once(|_| Err(()));
        let cadency = Cadency::default().await;
        assert!(cadency.is_err());
    }

    #[tokio::test]
    async fn should_build_cadency_with_env_token() {
        let _m = get_lock(&MTX);

        let env_cxt = mock_env_read::var_context();
        env_cxt
            .expect()
            .return_once(|_| Ok(String::from("ENV_VAR_VALUE")));
        let test = Cadency::default().await;
        assert!(test.is_ok())
    }
}
