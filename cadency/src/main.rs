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
        Box::new(Fib),
        Box::new(Inspire),
        Box::new(Now),
        Box::new(Pause),
        Box::new(Ping),
        Box::new(Play),
        Box::new(Resume),
        Box::new(Skip),
        Box::new(Slap),
        Box::new(Stop),
        Box::new(Tracks),
        Box::new(Urban),
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
