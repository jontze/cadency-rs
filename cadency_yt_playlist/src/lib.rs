#[macro_use]
extern crate serde;

mod error;
mod models;
mod playlist;
mod ytdlp;

pub use error::YtPlaylistError;
pub use playlist::*;
