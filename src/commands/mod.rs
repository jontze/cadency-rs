use serenity::client::Context;

mod ping;

/// Submit global slash commands to the discord api.
/// As global commands are cached for 1 hour, the activation ca take some time.
/// For local testing it is recommended to create commandswith a guild scope.
pub async fn setup_commands(ctx: &Context) -> Result<(), serenity::Error> {
    let _ping_cmd = ping::create(ctx).await?;
    Ok(())
}
