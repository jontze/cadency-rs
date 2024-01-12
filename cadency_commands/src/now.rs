use cadency_core::{
    response::{Response, ResponseBuilder},
    utils::{self, voice::TrackMetaKey},
    CadencyCommand, CadencyError,
};
use serenity::{async_trait, client::Context, model::application::CommandInteraction};
use songbird::tracks::LoopState;

#[derive(CommandBaseline, Default)]
#[description = "Shows current song"]
pub struct Now {}

#[async_trait]
impl CadencyCommand for Now {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
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

        // Extract Loop State from Track
        let loop_state = track.get_info().await.unwrap().loops;

        // Create message from track metadata. This is scoped to drop the read lock on the
        // trackmeta as soon as possible.
        let message = {
            let track_map = track.typemap().read().await;
            let metadata = track_map
                .get::<TrackMetaKey>()
                .expect("Metadata to be present in track map");

            metadata.title.as_ref().map_or(
                String::from(":x: **Could not add audio source to the queue!**"),
                |title| {
                    let mut track_info = format!(":newspaper: `{title}`");
                    match loop_state {
                        LoopState::Infinite => {
                            track_info.push_str("\n:repeat: `Infinite`");
                        }
                        LoopState::Finite(loop_amount) => {
                            if loop_amount > 0 {
                                track_info.push_str(&format!("\n:repeat: `{}`", loop_amount));
                            }
                        }
                    }
                    track_info
                },
            )
        };

        Ok(response_builder.message(Some(message)).build()?)
    }
}
