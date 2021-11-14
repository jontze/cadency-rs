#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serenity;
extern crate tokio;

mod client;
mod commands;
mod handler;

use client::create_client;

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = std::env::var("DISCORD_TOKEN").expect("Token in environment");

    let mut client = create_client(token).await.expect("Client to be created");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
