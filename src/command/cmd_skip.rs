use crate::storage::PlayListStore;
use crate::VoiceManager;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

//skip currently playing song
#[command]
fn skip(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(&ctx.http, "Not allowed in dms")?;
            return Ok(());
        }
    };

    let bot_guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            msg.channel_id
                .say(&ctx.http, "Error finding channel info")?;
            return Ok(());
        }
    };

    //get voice channels in the server
    let user_voice_channel_id = match guild
        .read()
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id)
    {
        Some(channel) => channel,
        None => {
            msg.reply(&ctx, "You are not in a voice channel")?;
            return Ok(());
        }
    };

    let manager_lock = ctx
        .data
        .read()
        .get::<VoiceManager>()
        .cloned()
        .expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    let handler = match manager.get_mut(bot_guild_id) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx, "I'm not playing any song")?;
            return Ok(());
        }
    };

    match handler.channel_id {
        None => {
            msg.reply(&ctx, "I'm not playing any song")?;
            return Ok(());
        }
        Some(id) => {
            if id != user_voice_channel_id {
                msg.reply(&ctx, "I'm not playing any song in your voice channel")?;
                return Ok(());
            }
        }
    }

    let music = ctx
        .data
        .read()
        .get::<PlayListStore>()
        .cloned()
        .expect("Can't access playlist");
    let mut music = music.lock();

    if let Some(playlist) = music.get_mut(&bot_guild_id) {
        if let Some(audio) = playlist.pop_front() {
            audio.lock().pause();
            msg.channel_id.say(&ctx.http, "Skipping!")?;
            if playlist.is_empty() {
                music.remove(&bot_guild_id);
                handler.stop();
                manager.remove(bot_guild_id);
            }
        } else {
            msg.reply(&ctx, "Nothing to skip!")?;
        }
    } else {
        msg.reply(&ctx, "I'm not playing any song")?;
    }
    
    Ok(())
}
