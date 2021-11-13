use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _new_message: Message) {
        println!("{:?}", _new_message);
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expect token in environment");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Should on creating client");

    let start = client.start().await;
    match start {
        Ok(()) => println!("Start Cadency-rs client..."),
        Err(start_error) => println!("Client start error: {:?}", start_error),
    }
}
