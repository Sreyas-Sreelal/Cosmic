use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::torrent::types::TorrentClient;
use crate::utils::*;

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

    if pageinfo.description.len() <= 2000 {
        send_success_msg(
            msg.channel_id,
            Some(&pageinfo.name),
            &pageinfo.description,
            &ctx.http,
        )?;
    } else {
        send_warn_msg(
            msg.channel_id,
            Some(&pageinfo.name),
            &format!(
                "The description is too long to send in discord\nRefer it here {}",
                pageinfo.url
            ),
            &ctx.http,
        )?;
    }

    send_success_msg(
        msg.channel_id,
        Some("magnet url"),
        &pageinfo.magnet_url,
        &ctx.http,
    )?;

    Ok(())
}
