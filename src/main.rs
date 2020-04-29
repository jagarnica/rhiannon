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

/// Serenity Handle
struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        println!("Received msg: {:#?}", msg);
        if msg.content == "!ping" {
            // msg.channel_id
            msg.reply(&ctx.http, "pong!")
                .map_err(|why| println!("Send error: {:#?}", why));
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected. \nData: {:#?}", ready.user.name, ready);
    }
}

fn main() {
    let discord_token = env::var("DISCORD_TOKEN").expect("No discord token found!");

    let mut client = Client::new(&discord_token, Handler).expect("Error creating client");

    if let Err(why) = client.start() {
        println!("Client error: {}", why);
    }

    // enumerate all available input formats
    audioproccesing::inputs::enumerate_device_info();

}
