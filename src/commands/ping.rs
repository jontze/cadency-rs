use serenity::{client::Context, model::interactions::application_command::ApplicationCommand};

pub async fn create(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
    Ok(
        ApplicationCommand::create_global_application_command(&ctx.http, |command| {
            command.name("ping").description("Play Ping-Pong")
        })
        .await?,
    )
}
