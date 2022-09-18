#[derive(thiserror::Error, Debug)]
pub enum YtPlaylistError {
    #[error("Unexpected command return")]
    CommandFailure(#[from] std::io::Error),
    #[error("Could not parse output to string")]
    OutPutParsing(#[from] std::string::FromUtf8Error),
    #[error("Could not serialize response")]
    Deserializing {
        source: serde_json::Error,
        message: String,
        data: String,
    },
}
