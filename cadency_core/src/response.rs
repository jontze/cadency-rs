use crate::CadencyError;
use derive_builder::Builder;
use serenity::{
    builder::{
        CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
        EditInteractionResponse,
    },
    model::prelude::CommandInteraction,
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
        command: &'a mut CommandInteraction,
    ) -> Result<(), CadencyError> {
        match self.timing {
            // Create a regular text response that might has embeds
            ResponseTiming::Instant => {
                let response = if let Some(msg) = self.message {
                    CreateInteractionResponseMessage::new()
                        .content(msg)
                        .add_embeds(self.embeds)
                } else {
                    CreateInteractionResponseMessage::new().add_embeds(self.embeds)
                };
                command
                    .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                    .await
            }
            // Just indicate that the command is being processed
            ResponseTiming::DeferredInfo => {
                command
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
                    )
                    .await
            }
            // Edit the deferred response with the actual response
            ResponseTiming::Deferred => {
                let edit_response = if let Some(msg) = self.message {
                    EditInteractionResponse::new()
                        .content(msg)
                        .add_embeds(self.embeds)
                } else {
                    EditInteractionResponse::new().add_embeds(self.embeds)
                };
                command
                    .edit_response(&ctx.http, edit_response)
                    .await
                    .map(|_| ())
            }
        }
        .map_err(|err| {
            error!("Failed to submit response: {}", err);
            CadencyError::Response
        })?;
        Ok(())
    }
}
