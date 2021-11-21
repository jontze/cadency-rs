use super::Command;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateInteractionResponse},
    client::Context,
    model::interactions::{
        application_command::{
            ApplicationCommand, ApplicationCommandInteraction,
            ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
        },
        InteractionResponseType,
    },
    utils::Color,
};

#[derive(serde::Deserialize, Debug)]
struct UrbanEntry {
    pub definition: String,
    pub permalink: String,
    pub thumbs_up: i64,
    pub sound_urls: Vec<String>,
    pub author: String,
    pub word: String,
    pub defid: i64,
    pub current_vote: String,
    pub written_on: String,
    pub example: String,
    pub thumbs_down: i64,
}

#[derive(serde::Deserialize, Debug)]
struct UrbanResult {
    pub list: Vec<UrbanEntry>,
}

pub struct Urban;

impl Urban {
    async fn request_urban_dictionary_entries(
        query: &str,
    ) -> Result<Vec<UrbanEntry>, reqwest::Error> {
        debug!("Requesting urban dictionary and deserialize json body");
        let url = format!("https://api.urbandictionary.com/v0/define?term={}", query);
        Ok(reqwest::get(url).await?.json::<UrbanResult>().await?.list)
    }

    fn response(
        response: &mut CreateInteractionResponse,
        urban_entries: Vec<UrbanEntry>,
    ) -> &mut CreateInteractionResponse {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                for (index, urban) in urban_entries.iter().enumerate() {
                    if index >= 3 {
                        break;
                    }
                    let mut embed_urban_entry = CreateEmbed::default();
                    embed_urban_entry.color(Color::from_rgb(255, 255, 0));
                    embed_urban_entry.title(&urban.word.replace("[", "").replace("]", ""));
                    embed_urban_entry.url(&urban.permalink);
                    embed_urban_entry.field(
                        "Definition",
                        &urban.definition.replace("[", "").replace("]", ""),
                        false,
                    );
                    embed_urban_entry.field(
                        "Example",
                        &urban.example.replace("[", "").replace("]", ""),
                        false,
                    );
                    embed_urban_entry.field(
                        "Rating",
                        format!(
                            "{} :thumbsup:  {} :thumbsdown:",
                            urban.thumbs_up, urban.thumbs_down
                        ),
                        false,
                    );
                    message.add_embed(embed_urban_entry);
                }
                message
            })
    }

    fn error_response<'a>(
        response: &'a mut CreateInteractionResponse,
        error_msg: &str,
    ) -> &'a mut CreateInteractionResponse {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(error_msg))
    }
}

#[async_trait]
impl Command for Urban {
    async fn register(ctx: &Context) -> Result<ApplicationCommand, serenity::Error> {
        Ok(
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("urban")
                    .description("Searches the Urbandictionary for your input")
                    .create_option(|option| {
                        option
                            .name("query")
                            .description("Your search query")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
            .await?,
        )
    }

    async fn execute(
        ctx: &Context,
        command: ApplicationCommandInteraction,
    ) -> Result<(), serenity::Error> {
        debug!("Execute urban command");
        let query_option =
            command
                .data
                .options
                .get(0)
                .and_then(|option| match option.resolved.as_ref() {
                    Some(value) => {
                        if let ApplicationCommandInteractionDataOptionValue::String(query) = value {
                            Some(query)
                        } else {
                            None
                        }
                    }
                    None => {
                        error!("Urban command option empty");
                        None
                    }
                });
        match query_option {
            Some(query) => {
                let urbans_entrys = Self::request_urban_dictionary_entries(query).await;
                match urbans_entrys {
                    Ok(urbans) => {
                        if urbans.is_empty() {
                            command
                                .create_interaction_response(&ctx.http, |res| {
                                    Self::error_response(res, ":x: *Nothing found*")
                                })
                                .await?
                        } else {
                            command
                                .create_interaction_response(&ctx.http, |res| {
                                    Self::response(res, urbans)
                                })
                                .await?;
                        }
                    }
                    Err(err) => {
                        error!("Failed to request urban dictionary entries : {:?}", err);
                        command
                            .create_interaction_response(&ctx.http, |res| {
                                Self::error_response(res, "Failed to request urban dictionary")
                            })
                            .await?;
                    }
                }
            }
            None => {
                error!("Urban empty query");
                command
                    .create_interaction_response(&ctx.http, |response| {
                        Self::error_response(response, "Empty or invalid query")
                    })
                    .await?;
            }
        };
        Ok(())
    }
}
