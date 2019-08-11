use crate::storage::PlayListStore;
use crate::utils::*;
use crate::VoiceManager;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

//stop playing song
#[command]
fn stop(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            send_error_msg(msg.channel_id, None, "Not allowed in dms", &ctx.http)?;
            return Ok(());
        }
    };

    let bot_guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            send_error_msg(
                msg.channel_id,
                None,
                "Error finding channel info",
                &ctx.http,
            )?;
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
    handler.stop();
    manager.remove(bot_guild_id);
    let playlist = ctx
        .data
        .read()
        .get::<PlayListStore>()
        .cloned()
        .expect("Can't access playlist");
    let mut playlist = playlist.lock();
    playlist.remove(&bot_guild_id);
    send_info_msg(msg.channel_id, None, "Stopped music!", &ctx.http)?;
    Ok(())
}
