use crate::handler::command::Handler;
use crate::intents::CadencyIntents;
use serenity::client::{Client, ClientBuilder};
#[cfg(feature = "audio")]
use songbird::SerenityInit;

pub struct Cadency {
    client: serenity::Client,
}

impl Cadency {
    /// Construct the Cadency discord but with default configuration
    pub async fn default() -> Result<Self, serenity::Error> {
        let token = std::env::var("DISCORD_TOKEN").expect("Token in environment");
        Self::new(token).await
    }

    /// Construct the Cadency discord but with a custom token that can be set programmatically
    ///
    /// # Arguments
    /// * `token` - The discord bot token as string
    pub async fn new(token: String) -> Result<Self, serenity::Error> {
        Ok(Self {
            client: Self::create_client(token).await?,
        })
    }

    /// This will actually start the configured Cadency bot
    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client.start().await
    }

    /// Setup the fundamental serenity client that is used for every feature
    ///
    /// # Arguments
    /// * `token` - The discord bot token as string
    async fn construct_client_baseline(token: String) -> ClientBuilder {
        Client::builder(token, CadencyIntents::default()).event_handler(Handler)
    }

    /// Create a ready to use serenity client instance
    ///
    /// # Arguments
    /// * `token` - The discord bot token as string
    #[cfg(not(feature = "audio"))]
    async fn create_client(token: String) -> Result<Client, serenity::Error> {
        let client = Self::construct_client_baseline(token).await;
        client.await
    }

    /// Create a ready to use serenity client instance with songbird audio  
    ///
    /// # Arguments
    /// * `token` - The discord bot token as string
    #[cfg(feature = "audio")]
    async fn create_client(token: String) -> Result<Client, serenity::Error> {
        info!("🎶 Audio feature enabled");
        let client = Self::construct_client_baseline(token)
            .await
            .intents(CadencyIntents::with_audio());
        client.register_songbird().await
    }
}
