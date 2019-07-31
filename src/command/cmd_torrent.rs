use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::torrent::types::TorrentClient;

//command say
//deletes the command message
//resends it as if bot if speaking!
#[command]
fn torrent(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let input = args.rest();
    let mut client = TorrentClient {
        proxy_link: String::new(),
    };

    let pageinfo = client.get_torrent_info(&input)?;
    let message = format!("```{}```", pageinfo.description);
    msg.channel_id.say(&ctx.http, message)?;

    let message = format!("***magnet url***\n```{}```", pageinfo.magnet_url);
    msg.channel_id.say(&ctx.http, message)?;

    Ok(())
}
