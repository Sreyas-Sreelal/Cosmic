use crate::storage::AIStore;
use crate::utils::remove_mention;

use log::info;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

//Handler
//Handles every incoming events in the bot
pub struct Handler;
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
