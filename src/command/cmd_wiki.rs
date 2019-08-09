use crate::http::*;
use serde_json::Value;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

//command wiki
//searches for wikipedia contents
#[command]
fn wiki(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let summary = get_wiki_summary(args.rest())?;
    msg.channel_id.say(&ctx.http, summary)?;
    Ok(())
}

pub fn get_wiki_summary(keyword: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=opensearch&search={}&limit=1&redirect=return",
        keyword
    );
    let request = HttpRequest {
        url,
        method: HttpMethod::Get,
        body: None,
    };

    let data = request.make_request()?;
    let wiki_data: Value = serde_json::from_str(&data)?;
    let summary: String = wiki_data.get(2).unwrap().get(0).unwrap().to_string();
    Ok(summary)
}
