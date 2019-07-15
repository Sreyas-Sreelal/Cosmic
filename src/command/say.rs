use serenity::framework::standard::{macros::command, Args, CommandResult, Delimiter};
use serenity::model::channel::Message;
use serenity::prelude::*;

//command say
//deletes the command message
//resends it as if bot if speaking!
#[command]
fn say(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    let _ = args.single::<String>();
    let to_say = args.rest();
    msg.delete(&ctx)?;
    msg.channel_id.say(&ctx.http, to_say)?;
    Ok(())
}
