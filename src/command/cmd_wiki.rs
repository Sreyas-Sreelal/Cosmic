use crate::wiki::api::get_wiki_summary;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

//command wiki
//searches for wikipedia contents
#[command]
fn wiki(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let wiki = get_wiki_summary(args.rest())?;
    msg.channel_id
        .say(&ctx.http, wiki.title + "\n" + &wiki.summary)?;
    Ok(())
}
