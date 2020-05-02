// mod discord - handle all the discord-related functionality
// TODO: create a way for custom functions to be called when certain events are
// recieved, as well as create/recieve audio

use serenity::{
    prelude::*,
    Client,
    model::{
        channel::Message,
        gateway::Ready
    },
};


/// Main API interface
pub struct Discord {
    client: Client,
}

impl Discord {
    /// Create a new discord handle with a bot token
    pub fn new(token: &String) -> Discord {
        Discord {
            client: Client::new(token, DiscordHandle).unwrap(),
        }
    }

    /// Start the bot, this function does not return unless there is an error
    pub fn start(&mut self) -> Result<(), String> {
        self.client
            .start()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Serenity event handle
struct DiscordHandle;

/// Recieve and process events from Discord
impl EventHandler for DiscordHandle {
    /// Called each time anyone send any message in the server
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong!") {
                println!("Send error: {}", why);
            }
        }
    }

    /// On connect
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected. \nData: {:#?}", ready.user.name, ready);
    }
}
