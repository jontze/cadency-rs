use crate::response::ResponseBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CadencyError {
    #[error("Missing environment variable '{0}'")]
    Environment(String),
    #[error("Failed to start cadency")]
    Start { source: serenity::Error },
    #[error("Failed to join a voice channel")]
    Join,
    #[error("Response failed")]
    Response,
    #[error("Command execution failed: {message}")]
    Command { message: String },
    #[error("Response building failed")]
    ResponseBuilder(#[from] ResponseBuilderError),
}
