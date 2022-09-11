#[macro_use]
extern crate log;

use cadency_commands::{
    Fib, Inspire, Now, Pause, Ping, Play, Resume, Skip, Slap, Stop, Tracks, Urban,
};
use cadency_core::{Cadency, CadencyCommand};

#[tokio::main]
async fn main() {
    env_logger::init();
    let commands: Vec<Box<dyn CadencyCommand>> = vec![
        Box::new(Fib::default()),
        Box::new(Inspire::default()),
        Box::new(Now::default()),
        Box::new(Pause::default()),
        Box::new(Ping::default()),
        Box::new(Play::default()),
        Box::new(Resume::default()),
        Box::new(Skip::default()),
        Box::new(Slap::default()),
        Box::new(Stop::default()),
        Box::new(Tracks::default()),
        Box::new(Urban::default()),
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
