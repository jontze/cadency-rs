#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;
#[macro_use]
extern crate serde;
extern crate serenity;
extern crate tokio;

mod client;
mod commands;
mod error;
mod handler;
mod utils;

use client::Cadency;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut cadency = Cadency::default().await.expect("To init Cadency");
    if let Err(why) = cadency.start().await {
        error!("Client error: {:?}", why);
    }
}
