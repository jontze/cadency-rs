use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError};
use serenity::{
    async_trait, builder::CreateEmbed, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

#[derive(CommandBaseline, Default)]
#[description = "List all tracks in the queue"]
#[deferred = true]
pub struct Tracks {
    options: Vec<CadencyCommandOption>,
}

#[async_trait]
impl CadencyCommand for Tracks {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        if let Some(guild_id) = command.guild_id {
            let manager = utils::voice::get_songbird(ctx).await;
            if let Some(call) = manager.get(guild_id) {
                let handler = call.lock().await;
                if handler.queue().is_empty() {
                    utils::voice::edit_deferred_response(
                        ctx,
                        command,
                        ":x: **No tracks in the queue**",
                    )
                    .await?;
                } else {
                    let queue_snapshot = handler.queue().current_queue();
                    let mut embeded_tracks = CreateEmbed::default();
                    embeded_tracks.color(Color::BLURPLE);
                    embeded_tracks.title("Track List");
                    for (index, track) in queue_snapshot.into_iter().enumerate() {
                        let position = index + 1;
                        let metadata = track.metadata();
                        let title = metadata
                            .title
                            .as_ref()
                            .map_or("**No title provided**", |t| t);
                        let url = metadata
                            .source_url
                            .as_ref()
                            .map_or("**No url provided**", |u| u);
                        embeded_tracks.field(
                            format!("{position}. :newspaper: `{title}`"),
                            format!(":notes: `{url}`"),
                            false,
                        );
                    }
                    utils::voice::edit_deferred_response_with_embeded(
                        ctx,
                        command,
                        vec![embeded_tracks],
                    )
                    .await?;
                }
            } else {
                utils::voice::edit_deferred_response(
                    ctx,
                    command,
                    ":x: **No tracks in the queue**",
                )
                .await?;
            }
        } else {
            utils::create_response(
                ctx,
                command,
                ":x: **This command can only be executed on a server**",
            )
            .await?;
        }
        Ok(())
    }
}
