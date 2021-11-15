use super::handler::Handler;
use serenity::{client::Client, http::Http};

/// Extract the user id of the current used bot from the discord api
///
/// # Arguments
/// * `token` - The discord bot token as string
async fn get_bot_id(token: &str) -> Result<serenity::model::id::UserId, serenity::Error> {
    let http = Http::new_with_token(token);
    let info = http.get_current_application_info().await?;
    Ok(info.id)
}

/// Create a ready to use serenity client instance
///
/// # Arguments
/// * `token` - The discord bot token as string
pub async fn create_client(token: String) -> Result<Client, serenity::Error> {
    let bot_id = get_bot_id(&token).await.expect("Bot id to be extracted");
    Client::builder(token)
        .event_handler(Handler)
        .application_id(bot_id.0)
        .await
}
