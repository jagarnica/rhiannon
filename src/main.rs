#![allow(unused)] // don't warn on unused things
use std::env;

use serenity::{
    self,
    prelude::*,
    Client,
    model::{
        channel::Message,
        gateway::Ready
    },
};

use cpal;
use cpal::traits::*;
mod audioproccesing;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong!") {
                println!("Send error: {}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected. \nData: {:#?}", ready.user.name, ready);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("No discord token found!");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    if let Err(why) = client.start() {
        println!("Client error: {}", why);
    }

    // // enumerate all available input formats
    // audioproccesing::inputs::list_inputs();
    // audioproccesing::inputs::list_hosts();
}
