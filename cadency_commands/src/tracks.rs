use cadency_core::{utils, CadencyCommand, CadencyError, CommandBaseline};
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::Context,
    model::application::{
        command::Command, interaction::application_command::ApplicationCommandInteraction,
    },
    utils::Color,
};

#[derive(CommandBaseline)]
pub struct Tracks;

#[async_trait]
impl CadencyCommand for Tracks {
    async fn register(&self, ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command
                    .name(self.name())
                    .description("List all tracks in the queue")
            })
            .await?,
        )
    }

    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute {} command", self.name());
        if let Some(guild_id) = command.guild_id {
            utils::voice::create_deferred_response(ctx, command).await?;
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
                    utils::voice::edit_deferred_response_with_embeded(ctx, command, embeded_tracks)
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
