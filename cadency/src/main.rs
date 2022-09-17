#[macro_use]
extern crate log;
#[macro_use]
extern crate cadency_core;

use cadency_commands::{
    Fib, Inspire, Now, Pause, Ping, Play, Resume, Skip, Slap, Stop, Tracks, Urban,
};
use cadency_core::Cadency;

#[tokio::main]
async fn main() {
    env_logger::init();

    let commands = setup_commands![
        Fib::default(),
        Inspire::default(),
        Now::default(),
        Pause::default(),
        Ping::default(),
        Play::default(),
        Resume::default(),
        Skip::default(),
        Slap::default(),
        Stop::default(),
        Tracks::default(),
        Urban::default(),
    ];
    let mut cadency = Cadency::default()
        .await
        .expect("To init Cadency")
        .with_commands(commands)
        .await;
    if let Err(why) = cadency.start().await {
        error!("Client error: {:?}", why);
    }
}
