use crate::error::CadencyError;
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
#[cfg(feature = "audio")]
pub mod now;
pub mod ping;
#[cfg(feature = "audio")]
pub mod play;
pub mod slap;
pub mod urban;

pub use fib::Fib;
pub use inspire::Inspire;
#[cfg(feature = "audio")]
pub use now::Now;
pub use ping::Ping;
#[cfg(feature = "audio")]
pub use play::Play;
pub use slap::Slap;
pub use urban::Urban;

#[async_trait]
pub trait Command {
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error>;
    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError>;
}

/// Submit global slash commands to the discord api.
/// As global commands are cached for 1 hour, the activation ca take some time.
/// For local testing it is recommended to create commandswith a guild scope.
pub async fn setup_commands(ctx: &Context) -> Result<(), serenity::Error> {
    tokio::try_join!(
        Ping::register(ctx),
        Inspire::register(ctx),
        Fib::register(ctx),
        Urban::register(ctx),
        Slap::register(ctx)
    )?;
    #[cfg(feature = "audio")]
    tokio::try_join!(Play::register(ctx), Now::register(ctx))?;
    Ok(())
}

pub async fn command_not_implemented(
    ctx: &Context,
    command: ApplicationCommandInteraction,
) -> Result<(), CadencyError> {
    error!("The following command is not known: {:?}", command);
    let unknown_command = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Unknown command"))
        })
        .await
        .map_err(|err| {
            error!("Interaction response failed: {}", err);
            CadencyError::Response
        })?;
    Ok(unknown_command)
}
