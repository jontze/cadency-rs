use crate::CadencyError;
use derive_builder::Builder;
use serenity::{
    builder::CreateEmbed,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
};

#[derive(Debug, Clone)]
pub enum ResponseTiming {
    Deferred,
    DeferredInfo,
    Instant,
}

#[derive(Builder)]
pub struct Response {
    timing: ResponseTiming,
    #[builder(default)]
    message: Option<String>,
    #[builder(default)]
    embeds: Vec<CreateEmbed>,
}

impl ResponseBuilder {
    pub fn new(timing: ResponseTiming) -> Self {
        Self {
            timing: Some(timing),
            ..Default::default()
        }
    }
}

impl Response {
    pub async fn submit<'a>(
        self,
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        match self.timing {
            ResponseTiming::Instant => {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                if let Some(msg) = self.message {
                                    message.content(msg);
                                }
                                message.add_embeds(self.embeds)
                            })
                    })
                    .await
            }
            ResponseTiming::DeferredInfo => {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                    })
                    .await
            }
            ResponseTiming::Deferred => command
                .edit_original_interaction_response(&ctx.http, |previous_response| {
                    if let Some(msg) = self.message {
                        previous_response.content(msg);
                    }
                    previous_response.add_embeds(self.embeds)
                })
                .await
                .map(|_| ()),
        }
        .map_err(|err| {
            error!("Failed to submit response: {}", err);
            CadencyError::Response
        })?;
        Ok(())
    }
}
