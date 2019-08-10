use crate::storage::{AIStore, PlayListStore, VoiceManager};
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
    fn ready(&self, ctx: Context, data: Ready) {
        info!("Connected to account : {}", data.user.name);
        std::thread::spawn(move || {
            let music = ctx
                .data
                .read()
                .get::<PlayListStore>()
                .cloned()
                .expect("Can't access playlist");

            let manager = ctx
                .data
                .read()
                .get::<VoiceManager>()
                .cloned()
                .expect("Expected VoiceManager in ShareMap.");

            loop {
                let music_list = music.try_lock();
                let manager = manager.try_lock();

                if music_list.is_some() && manager.is_some() {
                    for (_, v) in &mut music_list.unwrap().iter_mut() {
                        if let Some(audio) = &v.front() {
                            if audio.lock().finished {
                                let _ = v.pop_front();
                            } else if !audio.lock().playing {
                                audio.lock().play();
                                //channel_id.say(&ctx.http, "playing")?;
                            }
                        }
                    }
                }
            }
        });
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
