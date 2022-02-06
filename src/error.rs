use thiserror::Error;

#[derive(Error, Debug)]
pub enum CadencyError {
    #[cfg(feature = "audio")]
    #[error("Failed to join a voice channel")]
    Join,
    #[error("Response failed")]
    Response,
}
