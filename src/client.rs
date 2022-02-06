use crate::handler::command::Handler;
use serenity::{
    client::{Client, ClientBuilder},
    http::Http,
};
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

    /// Extract the user id of the current used bot from the discord api
    ///
    /// # Arguments
    /// * `token` - The discord bot token as string
    async fn get_bot_id(token: &str) -> Result<serenity::model::id::UserId, serenity::Error> {
        let http = Http::new_with_token(token);
        let info = http.get_current_application_info().await?;
        Ok(info.id)
    }

    /// Setup the fundamental serenity client that is used for every feature
    ///
    /// # Arguments
    /// * `token` - The discord bot token as string
    async fn construct_client_baseline<'a>(token: String) -> ClientBuilder<'a> {
        let bot_id = Self::get_bot_id(&token)
            .await
            .expect("Bot id to be extracted");
        Client::builder(token)
            .event_handler(Handler)
            .application_id(bot_id.0)
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
        let client = Self::construct_client_baseline(token).await;
        client.register_songbird().await
    }
}
