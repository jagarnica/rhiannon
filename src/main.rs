#![allow(unused)] // don't warn on unused things
use std::env;

use cpal;
use cpal::traits::*;
use serenity;

mod audioproccesing;
mod discord;
use discord::*;


fn main() {
    let discord_token = env::var("DISCORD_TOKEN").expect("No discord token found!");
    let mut discord   = Discord::new(&discord_token);

    if let Err(why) = discord.start() {
        println!("Client error: {}", why);
    }

    // enumerate all available input formats
    audioproccesing::inputs::enumerate_device_info();
}
