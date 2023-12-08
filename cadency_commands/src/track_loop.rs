use cadency_core::{
    response::{Response, ResponseBuilder},
    utils, CadencyCommand, CadencyError,
};
use serenity::{async_trait, client::Context, model::application::CommandInteraction};

#[derive(Default, CommandBaseline)]
#[name = "loop"]
#[description = "Loop the current track"]
#[argument(
    name = "amount",
    description = "The amount of times to loop the track",
    required = false,
    kind = "Integer"
)]
#[argument(
    name = "stop",
    description = "Cancel looping",
    required = false,
    kind = "Boolean"
)]
pub struct TrackLoop {}

#[async_trait]
impl CadencyCommand for TrackLoop {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        // Validate if command can be executed
        let guild_id = command.guild_id.ok_or(CadencyError::Command {
            message: ":x: **This command can only be executed on a server**".to_string(),
        })?;
        let manager = utils::voice::get_songbird(ctx).await;
        let call = manager.get(guild_id).ok_or(CadencyError::Command {
            message: ":x: **No active voice session on the server**".to_string(),
        })?;
        let handler = call.lock().await;
        let track = handler.queue().current().ok_or(CadencyError::Command {
            message: ":x: **No song is playing**".to_string(),
        })?;

        // Extract the loop amount and stop argument from the command
        let loop_amount = self.arg_amount(command);
        let stop_argument = self.arg_stop(command);

        // Cancel looping if the stop argument is true
        if let Some(stop) = stop_argument {
            if stop {
                track.disable_loop().map_err(|err| {
                    error!("Could not disable loop: {}", err);
                    CadencyError::Command {
                        message: ":x: **Could not disable loop**".to_string(),
                    }
                })?;
                return Ok(response_builder
                    .message(Some(":repeat: **Looping disabled**".to_string()))
                    .build()?);
            }
        }

        // Enable the loop infinite or for a specific amount of times
        let response_message = if let Some(amount) = loop_amount {
            track.loop_for(amount as usize).map_err(|err| {
                error!("Could not loop track '{amount}' times: {}", err);
                CadencyError::Command {
                    message: ":x: **Could not loop track**".to_string(),
                }
            })?;
            response_builder.message(Some(format!(":repeat: **Looping track `{amount}` times**")))
        } else {
            track.enable_loop().map_err(|err| {
                error!("Could not loop track infinite: {}", err);
                CadencyError::Command {
                    message: ":x: **Could not loop track**".to_string(),
                }
            })?;
            response_builder.message(Some(":repeat: **Looping track**".to_string()))
        }
        .build()?;
        Ok(response_message)
    }
}
