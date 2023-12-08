use crate::{
    command::{command_not_implemented, setup_commands},
    response::{ResponseBuilder, ResponseTiming},
    utils, CadencyError,
};
use serenity::{
    all::OnlineStatus,
    async_trait,
    client::{Context, EventHandler},
    gateway::ActivityData,
    model::{application::Interaction, event::ResumedEvent, gateway::Ready},
};

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        info!("🚀 Start Cadency Discord Bot");
        // Set the bot presence to "Listening to music"
        ctx.set_presence(Some(ActivityData::listening("music")), OnlineStatus::Online);

        info!("⏳ Started to submit commands, please wait...");
        match setup_commands(&ctx).await {
            Ok(()) => info!("✅ Application commands submitted"),
            Err(err) => error!("❌ Failed to submit application commands: {:?}", err),
        };
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        debug!("🔌 Reconnect to server");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(mut command) = interaction {
            let cmd_target = utils::get_commands(&ctx)
                .await
                .into_iter()
                .find(|cadency_command| cadency_command.name() == command.data.name.as_str());

            if let Some(cmd) = cmd_target {
                info!("⚡ Execute '{}' command", cmd.name());
                let mut prepared_response = ResponseBuilder::new(ResponseTiming::Instant);
                if cmd.deferred() {
                    prepared_response.timing(ResponseTiming::Deferred);
                    ResponseBuilder::new(ResponseTiming::DeferredInfo)
                        .build()
                        .expect("Failed to build response")
                        .submit(&ctx, &mut command)
                        .await
                        .expect("Unable to submit deferred info");
                }
                match cmd
                    .execute(&ctx, &mut command, &mut prepared_response)
                    .await
                {
                    Ok(response) => {
                        response
                            .submit(&ctx, &mut command)
                            .await
                            .expect("To submit the command response");
                        info!("✅ Command '{}' was successful", cmd.name());
                    }
                    Err(command_error) => {
                        error!("❌ Command execution failed: {command_error:?}");
                        let mut error_res_builder = ResponseBuilder::default();
                        if cmd.deferred() {
                            error_res_builder.timing(ResponseTiming::Deferred);
                        } else {
                            error_res_builder.timing(ResponseTiming::Instant);
                        }
                        match command_error {
                            CadencyError::Command { message } => {
                                error_res_builder.message(Some(message))
                            }
                            CadencyError::Join => error_res_builder.message(Some(
                                "❌ **I could not join your voice channel**".to_string(),
                            )),
                            _ => error_res_builder.message(Some(
                                "**Oops! Something went terrible wrong.**".to_string(),
                            )),
                        }
                        .build()
                        .expect("Unable to build error response")
                        .submit(&ctx, &mut command)
                        .await
                        .map_err(|err| {
                            error!("❌ Fatal error! Is discord down? {:?}", err);
                        })
                        .expect("Unable to send error response");
                    }
                }
            } else {
                command_not_implemented(&ctx, &command)
                    .await
                    .expect("Failed to submit not-implemented error");
            }
        };
    }
}
