#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;
#[macro_use]
extern crate serde;
extern crate serenity;

pub mod client;
mod commands;
mod error;
mod handler;
mod intents;
mod utils;
