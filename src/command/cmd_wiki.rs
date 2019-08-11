use crate::utils::*;
use crate::wiki::api::get_wiki_summary;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

//command wiki
//searches for wikipedia contents
#[command]
fn wiki(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let wiki = get_wiki_summary(args.rest())?;
    println!("{} \n {}", wiki.title, wiki.summary);
    if wiki.summary.len() <= 2000 {
        send_success_msg(msg.channel_id, Some(&wiki.title), &wiki.summary, &ctx.http)?;
    } else {
        send_error_msg(
            msg.channel_id,
            None,
            "Too long summary to send in discord :(",
            &ctx.http,
        )?;
    }
    Ok(())
}
