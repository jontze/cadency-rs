#[macro_use]
extern crate log;
#[macro_use]
extern crate cadency_core;

use cadency_commands::{
    Fib, Inspire, Now, Pause, Ping, Play, Resume, Roll, Skip, Slap, Stop, TrackLoop, Tracks, Urban,
};
use cadency_core::Cadency;
use settings::CadencySettings;

mod settings;

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "cadency=info");
    env_logger::init_from_env(env);

    let settings = CadencySettings::parse();

    let commands = setup_commands![
        Fib::default(),
        Inspire::default(),
        Now::default(),
        Pause::default(),
        Ping::default(),
        Play::new(
            settings.play.playlist_song_limit,
            settings.play.song_length_limit
        ),
        Resume::default(),
        Skip::default(),
        Slap::default(),
        Stop::default(),
        Tracks::default(),
        Urban::default(),
        TrackLoop::default(),
        Roll::default()
    ];
    let cadency = Cadency::builder()
        .token(std::env::var("DISCORD_TOKEN").expect("Discord token to be present"))
        .commands(commands)
        .build()
        .expect("To build cadency");

    if let Err(why) = cadency.start().await {
        error!("Client error: {:?}", why);
    }
}
