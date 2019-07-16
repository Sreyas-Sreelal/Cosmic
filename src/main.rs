mod command;
mod http;
mod imgflip;
use command::{ADMIN_GROUP, GENERAL_GROUP};

use log::{error, info};
use serenity::framework::StandardFramework;
use serenity::{model::gateway::Ready, prelude::*};

//Handler
//Handles every incoming events in the bot
struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, data: Ready) {
        info!("Connected to account : {}", data.user.name);
    }
}

fn main() -> Result<(), log::SetLoggerError> {
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
