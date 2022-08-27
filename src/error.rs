use thiserror::Error;

#[derive(Error, Debug)]
pub enum CadencyError {
    #[error("Failed to join a voice channel")]
    Join,
    #[error("Response failed")]
    Response,
}
