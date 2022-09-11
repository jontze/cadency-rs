use cadency_core::{utils, CadencyCommand, CadencyCommandOption, CadencyError, CommandBaseline};
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
    utils::Color,
};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

#[derive(CommandBaseline)]
pub struct Urban {
    description: &'static str,
    options: Vec<CadencyCommandOption>,
}

impl std::default::Default for Urban {
    fn default() -> Self {
        Self {
            description: "Searches the Urbandictionary for your input",
            options: vec![CadencyCommandOption {
                name: "url",
                description: "Your search query",
                kind: CommandOptionType::String,
                required: true,
            }],
        }
    }
}

impl Urban {
    async fn request_urban_dictionary_entries(
        query: &str,
    ) -> Result<Vec<UrbanEntry>, reqwest::Error> {
        debug!("Requesting urban dictionary and deserialize json body");
        let url = format!("https://api.urbandictionary.com/v0/define?term={}", query);
        Ok(reqwest::get(url).await?.json::<UrbanResult>().await?.list)
    }

    fn create_embed(urban_entries: Vec<UrbanEntry>) -> Vec<CreateEmbed> {
        let mut embeds: Vec<CreateEmbed> = Vec::new();
        for (index, urban) in urban_entries.iter().enumerate() {
            if index >= 3 {
                break;
            }
            let mut embed_urban_entry = CreateEmbed::default();
            embed_urban_entry.color(Color::from_rgb(255, 255, 0));
            embed_urban_entry.title(&urban.word.replace('[', "").replace(']', ""));
            embed_urban_entry.url(&urban.permalink);
            embed_urban_entry.field(
                "Definition",
                &urban.definition.replace('[', "").replace(']', ""),
                false,
            );
            embed_urban_entry.field(
                "Example",
                &urban.example.replace('[', "").replace(']', ""),
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
            embeds.push(embed_urban_entry);
        }
        embeds
    }
}

#[async_trait]
impl CadencyCommand for Urban {
    #[command]
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        let query_option =
            command
                .data
                .options
                .get(0)
                .and_then(|option| match option.resolved.as_ref() {
                    Some(value) => {
                        if let CommandDataOptionValue::String(query) = value {
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
                            utils::create_response(ctx, command, ":x: *Nothing found*").await?;
                        } else {
                            utils::create_response_with_embed(
                                ctx,
                                command,
                                Self::create_embed(urbans),
                            )
                            .await?;
                        }
                    }
                    Err(err) => {
                        error!("Failed to request urban dictionary entries : {:?}", err);
                        utils::create_response(
                            ctx,
                            command,
                            ":x: *Failed to request urban dictionary*",
                        )
                        .await?;
                    }
                }
            }
            None => {
                error!("Urban empty query");
                utils::create_response(ctx, command, ":x: *Empty or invalid query*").await?;
            }
        };
        Ok(())
    }
}
