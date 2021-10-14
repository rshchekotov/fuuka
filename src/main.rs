use serenity::prelude::Client;
use std::env;

mod environ;
mod event_handler;

#[tokio::main]
async fn main() {
    environ::load_env();

    let token = env::var("BOT_TOKEN").ok().unwrap();
    let application_id: u64 = env::var("DISCORD_APP_ID")
        .ok()
        .unwrap()
        .parse::<u64>()
        .expect("Application ID is not a valid ID!");

    let mut client = Client::builder(token)
        .event_handler(event_handler::Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
