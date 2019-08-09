mod ai;
mod command;
mod http;
mod imgflip;
mod torrent;

use crate::ai::AI;
use crate::command::{ADMIN_GROUP, GENERAL_GROUP};

use lazy_static::lazy_static;
use log::{error, info};
use regex::Regex;
use serenity::framework::StandardFramework;
use serenity::{
    client::bridge::voice::ClientVoiceManager,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env, sync::Arc};

//Handler
//Handles every incoming events in the bot
struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, data: Ready) {
        info!("Connected to account : {}", data.user.name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        info!("<{}> : {}", msg.author.name, msg.content);
        let brain = ctx
            .data
            .read()
            .get::<AIStore>()
            .cloned()
            .expect("Expected brain in ShareMap.");
        if let Ok(user) = ctx.http.get_current_user() {
            if msg.mentions_user_id(user.id) {
                let input = remove_mention(&msg.content);
                let mut brain = brain.lock();
                let response = brain.generate_response(&input);
                msg.channel_id.say(&ctx.http, response).unwrap();
            }
        }
    }
}

struct VoiceManager;
impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct AIStore;
impl TypeMapKey for AIStore {
    type Value = Arc<Mutex<AI>>;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Initialise log and set trace level
    env_logger::init();

    let token = env::var("COSMIC_TOKEN")
        .expect("Error cannot fetch token.Make sure environment variable COSMIC_TOKEN is set");

    let mut client = Client::new(&token, Handler).expect("Error Cannot build client object");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<AIStore>(Arc::new(Mutex::new(AI::new("cosmic_brain.json")?)));
    }

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

//removes mentions from the message
fn remove_mention(msg: &str) -> String {
    lazy_static! {
        static ref MENTION_RE: Regex = Regex::new("<@[0-9]+>").unwrap();
    }

    MENTION_RE.replace_all(&msg, "").to_string()
}
