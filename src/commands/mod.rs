use crate::error::CadencyError;
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
};

pub mod fib;
pub mod inspire;

pub mod now;

pub mod pause;
pub mod ping;

pub mod play;

pub mod resume;

pub mod skip;
pub mod slap;

pub mod stop;

pub mod tracks;
pub mod urban;

pub use fib::Fib;
pub use inspire::Inspire;

pub use now::Now;

pub use pause::Pause;
pub use ping::Ping;

pub use play::Play;

pub use resume::Resume;

pub use skip::Skip;
pub use slap::Slap;

pub use stop::Stop;

pub use tracks::Tracks;
pub use urban::Urban;

#[async_trait]
pub trait CadencyCommand {
    async fn register(ctx: &Context) -> Result<Command, serenity::Error>;
    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError>;
}

/// Submit global slash commands to the discord api.
/// As global commands are cached for 1 hour, the activation ca take some time.
/// For local testing it is recommended to create commands with a guild scope.
pub async fn setup_commands(ctx: &Context) -> Result<(), serenity::Error> {
    tokio::try_join!(
        Ping::register(ctx),
        Inspire::register(ctx),
        Fib::register(ctx),
        Urban::register(ctx),
        Slap::register(ctx)
    )?;

    tokio::try_join!(
        Play::register(ctx),
        Now::register(ctx),
        Skip::register(ctx),
        Pause::register(ctx),
        Resume::register(ctx),
        Stop::register(ctx),
        Tracks::register(ctx)
    )?;
    Ok(())
}

pub async fn command_not_implemented(
    ctx: &Context,
    command: ApplicationCommandInteraction,
) -> Result<(), CadencyError> {
    error!("The following command is not known: {:?}", command);
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Unknown command"))
        })
        .await
        .map_err(|err| {
            error!("Interaction response failed: {}", err);
            CadencyError::Response
        })
}
