#[macro_use] extern crate serenity;
#[macro_use] extern crate std;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(include_str!("../token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .cmd("ping", ping)
        .cmd("invite", invite));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, msg) {
    let _ = msg.channel_id.say("Pong!");
});

command!(invite(_context, msg) {
    let _ = msg.channel_id.say("https://discordapp.com/api/oauth2/authorize?client_id=570017324460015616&permissions=2112&scope=bot");
});