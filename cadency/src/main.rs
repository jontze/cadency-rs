#[macro_use]
extern crate log;

use cadency_core::client::Cadency;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut cadency = Cadency::default().await.expect("To init Cadency");
    if let Err(why) = cadency.start().await {
        error!("Client error: {:?}", why);
    }
}
