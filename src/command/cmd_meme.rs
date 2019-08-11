use crate::imgflip::api::get_memes;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

//command meme
//gets random meme from imgflip
#[command]
fn meme(ctx: &mut Context, msg: &Message) -> CommandResult {
    let meme = get_memes()?;
    msg.channel_id.say(&ctx.http, meme.url)?;
    Ok(())
}
