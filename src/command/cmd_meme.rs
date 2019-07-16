use crate::imgflip::api::get_memes;
use log::debug;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
//command meme
//gets random meme from imgflip
#[command]
fn meme(ctx: &mut Context, msg: &Message) -> CommandResult {
    debug!("cmd meme called");
    let meme = get_memes()?;
    debug!("meme img url {}", meme.url);
    msg.channel_id.say(&ctx.http, meme.url)?;
    Ok(())
}
