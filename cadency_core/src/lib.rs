#[macro_use]
extern crate log;
extern crate serenity;

pub mod client;
pub use client::Cadency;
mod command;
pub use command::{CadencyCommand, CadencyCommandOption, CommandBaseline};
mod error;
pub use error::CadencyError;
pub mod handler;
mod intents;
pub mod utils;
