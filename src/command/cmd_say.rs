use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

//command say
//deletes the command message
//resends it as if bot if speaking!
#[command]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let to_say = args.rest();
    msg.delete(&ctx)?;
    msg.channel_id.say(&ctx.http, to_say)?;
    Ok(())
}
