use cadency_core::{
    response::{Response, ResponseBuilder},
    utils, CadencyCommand, CadencyError,
};
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::Context,
    model::application::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOptionValue,
    },
    utils::Color,
};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UrbanEntry {
    pub definition: String,
    pub permalink: String,
    pub thumbs_up: i64,
    pub author: String,
    pub word: String,
    pub defid: i64,
    pub written_on: String,
    pub example: String,
    pub thumbs_down: i64,
}

#[derive(serde::Deserialize, Debug)]
struct UrbanResult {
    pub list: Vec<UrbanEntry>,
}

#[derive(CommandBaseline, Default)]
#[description = "Searches the Urbandictionary for your query"]
#[deferred = true]
#[argument(name = "query", description = "Your search query", kind = "String")]
pub struct Urban {}

impl Urban {
    async fn request_urban_dictionary_entries(
        query: &str,
    ) -> Result<Vec<UrbanEntry>, reqwest::Error> {
        debug!("Requesting urban dictionary and deserialize json body");
        let url = format!("https://api.urbandictionary.com/v0/define?term={query}");
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
            embed_urban_entry.title(&urban.word.replace(['[', ']'], ""));
            embed_urban_entry.url(&urban.permalink);
            embed_urban_entry.field(
                "Definition",
                &urban.definition.replace(['[', ']'], ""),
                false,
            );
            embed_urban_entry.field("Example", &urban.example.replace(['[', ']'], ""), false);
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
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
        respone_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, CadencyError> {
        let query = utils::get_option_value_at_position(command.data.options.as_ref(), 0)
            .and_then(|option_value| {
                if let CommandDataOptionValue::String(query) = option_value {
                    Some(query)
                } else {
                    error!("Urban command option empty");
                    None
                }
            })
            .ok_or(CadencyError::Command {
                message: ":x: *Empty or invalid query*".to_string(),
            })?;
        let urbans = Self::request_urban_dictionary_entries(query)
            .await
            .map_err(|err| {
                error!("Failed to request urban dictionary entries : {:?}", err);
                CadencyError::Command {
                    message: ":x: *Failed to request urban dictionary*".to_string(),
                }
            })?;
        let respone_builder = if urbans.is_empty() {
            respone_builder.message(Some(":x: *Nothing found*".to_string()))
        } else {
            respone_builder.embeds(Self::create_embed(urbans))
        };
        Ok(respone_builder.build()?)
    }
}
