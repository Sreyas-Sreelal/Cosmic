mod ai;
mod command;
mod http;
mod imgflip;

use crate::ai::AI;
use crate::command::{ADMIN_GROUP, GENERAL_GROUP};

use lazy_static::lazy_static;
use log::{error, info};
use serenity::framework::StandardFramework;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::sync::Mutex;

//Due to conflict in mutability in respond method in eliza and message trait in serenity
//declare BRAIN in global state
lazy_static! {
    static ref BRAIN: Mutex<AI> = Mutex::new(AI::new("cosmic_brain.json").unwrap());
}

//Handler
//Handles every incoming events in the bot
struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, data: Ready) {
        info!("Connected to account : {}", data.user.name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        info!("<{}> : {}", msg.author.name, msg.content);

        if let Ok(user) = ctx.http.get_current_user() {
            if msg.mentions_user_id(user.id) {
                let response = BRAIN.lock().unwrap().respond(&msg.content);
                msg.channel_id.say(&ctx.http, response).unwrap();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Initialise log and set trace level
    env_logger::init();

    let token = std::env::var("COSMIC_TOKEN")
        .expect("Error cannot fetch token.Make sure environment variable COSMIC_TOKEN is set");

    let mut client = Client::new(&token, Handler).expect("Error Cannot build client object");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("$"))
            .group(&ADMIN_GROUP)
            .group(&GENERAL_GROUP),
    );

    if let Err(error) = client.start() {
        error!("{}", error);
    }

    Ok(())
}
