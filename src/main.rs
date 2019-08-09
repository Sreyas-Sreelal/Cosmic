mod ai;
mod command;
mod events;
mod http;
mod imgflip;
mod storage;
mod torrent;
mod utils;

use crate::ai::AI;
use crate::command::{ADMIN_GROUP, GENERAL_GROUP};
use crate::events::Handler;
use crate::storage::{AIStore, VoiceManager};

use log::error;
use serenity::{framework::StandardFramework, prelude::*};
use std::{env, sync::Arc};

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
