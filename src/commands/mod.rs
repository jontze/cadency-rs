use serenity::{
    async_trait,
    client::Context,
    model::interactions::{
        application_command::{ApplicationCommand, ApplicationCommandInteraction},
        InteractionResponseType,
    },
};

pub mod fib;
pub mod inspire;
pub mod ping;
#[cfg(feature = "audio")]
pub mod play;
pub mod slap;
pub mod urban;

pub use fib::Fib;
pub use inspire::Inspire;
pub use ping::Ping;
#[cfg(feature = "audio")]
pub use play::Play;
pub use slap::Slap;
pub use urban::Urban;

#[async_trait]
pub trait Command {
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error>;
    async fn execute(
        ctx: &Context,
        command: ApplicationCommandInteraction,
    ) -> Result<(), serenity::Error>;
}

/// Submit global slash commands to the discord api.
/// As global commands are cached for 1 hour, the activation ca take some time.
/// For local testing it is recommended to create commandswith a guild scope.
pub async fn setup_commands(ctx: &Context) -> Result<(), serenity::Error> {
    let _ping_cmd = Ping::register(ctx).await?;
    let _inspire_cmd = Inspire::register(ctx).await?;
    let _fib_cmd = Fib::register(ctx).await?;
    let _urban_cmd = Urban::register(ctx).await?;
    let _slap_cmd = Slap::register(ctx).await?;
    if cfg!(feature = "audio") {
        let _play_cmd = Play::register(ctx).await?;
    }
    Ok(())
}

pub async fn command_not_implemented(
    ctx: &Context,
    command: ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    error!("The following command is not known: {:?}", command);
    let unknown_command = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Unknown command"))
        })
        .await?;
    Ok(unknown_command)
}
