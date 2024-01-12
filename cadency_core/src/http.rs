use serenity::prelude::TypeMapKey;

pub struct HttpClientKey;

impl TypeMapKey for HttpClientKey {
    type Value = reqwest::Client;
}

pub(crate) async fn get_http_client(ctx: &serenity::client::Context) -> reqwest::Client {
    let data = ctx.data.read().await;
    data.get::<HttpClientKey>()
        .expect("Expected HttpClientKey in TypeMap.")
        .clone()
}
