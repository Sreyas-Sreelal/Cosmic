use log::{error, info, Level, Metadata, Record};
use serenity::{model::gateway::Ready, prelude::*};

//Handler
//Handles every incoming events in the bot
struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, data: Ready) {
        info!("Connected to account : {}", data.user.name);
    }
}

//Logger
//A basic logger for the bot
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[Cosmic] [{}] : {} ", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static COMSIC_LOGGER: Logger = Logger;

fn main() -> Result<(), log::SetLoggerError> {
    //Initialise log and set trace level
    log::set_logger(&COMSIC_LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info))?;

    let token = std::env::var("COSMIC_TOKEN")
        .expect("Error cannot fetch token.Make sure environment variable COSMIC_TOKEN is set");
    let mut client = Client::new(&token, Handler).expect("Error Cannot build client object");
    if let Err(error) = client.start() {
        error!("{}", error);
    }
    Ok(())
}
