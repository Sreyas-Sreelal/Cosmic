use crate::storage::PlayListStore;
use crate::VoiceManager;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::voice;
use std::collections::VecDeque;

//plays a song by name
#[command]
fn play(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.rest();

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

    let user_guild_id = guild.read().id;

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

    match manager.get_mut(bot_guild_id) {
        Some(currrent_handler) => {
            let bot_voice_channel_id = currrent_handler.channel_id;
            if bot_voice_channel_id.is_none() {
                if manager.join(user_guild_id, user_voice_channel_id).is_none() {
                    msg.channel_id.say(&ctx.http, "Error joining the channel")?;
                    return Ok(());
                }
            } else {
                let bot_voice_channel_id = bot_voice_channel_id.unwrap();
                if bot_voice_channel_id != user_voice_channel_id {
                    msg.reply(
                        &ctx,
                        &format!(
                            "Im already playing a song in {}",
                            bot_voice_channel_id.mention()
                        ),
                    )?;
                    return Ok(());
                }
            }
        }
        None => {
            if manager.join(user_guild_id, user_voice_channel_id).is_none() {
                msg.channel_id.say(&ctx.http, "Error joining the channel")?;
                return Ok(());
            }
        }
    }

    //voice::ffmpeg(path: P)
    let source = match voice::ytdl_search(&name) {
        Ok(source) => source,
        Err(why) => {
            println!("Err starting source: {:?}", why);
            msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg")?;
            return Ok(());
        }
    };
    let playlist = ctx
        .data
        .read()
        .get::<PlayListStore>()
        .cloned()
        .expect("Can't access playlist");

    let mut playlist = playlist.lock();
    let music_handler = manager.get_mut(bot_guild_id);
    let handler = music_handler.unwrap();
    match playlist.get_mut(&user_guild_id) {
        None => {
            msg.channel_id.say(&ctx.http, "Playing song")?;
            handler.stop();
            let locked_audio = handler.play_returning(source);
            let mut queue: VecDeque<voice::LockedAudio> = VecDeque::new();
            queue.push_back(locked_audio);
            playlist.insert(user_guild_id, queue);
        }
        Some(music) => {
            let locked_audio = handler.play_returning(source);
            locked_audio.lock().pause();
            music.push_back(locked_audio);
            msg.channel_id.say(&ctx.http, "Adding to queue")?;
        }
    }

    Ok(())
}
