use lazy_static::lazy_static;
use regex::Regex;
use serenity::http::raw::Http;
use serenity::model::id::ChannelId;
use serenity::utils::Colour;

//removes mentions from the message
pub fn remove_mention(msg: &str) -> String {
    lazy_static! {
        static ref MENTION_RE: Regex = Regex::new("<@[0-9]+>").unwrap();
    }
    MENTION_RE.replace_all(&msg, "").to_string()
}

//wrappers for sending embeds
fn send_message_with_embed(
    color: Colour,
    channel_id: ChannelId,
    title: Option<&str>,
    content: &str,
    http: &Http,
) -> Result<(), Box<dyn std::error::Error>> {
    channel_id.send_message(http, |m| {
        m.embed(|e| {
            if let Some(title) = title {
                e.title(title);
            }
            e.description(content);
            e.colour(color);
            e
        });
        m
    })?;
    Ok(())
}

pub fn send_warn_msg(
    channel_id: ChannelId,
    title: Option<&str>,
    content: &str,
    http: &Http,
) -> Result<(), Box<dyn std::error::Error>> {
    let colour = Colour::new(0xFFFF00);
    send_message_with_embed(colour, channel_id, title, content, http)?;
    Ok(())
}

pub fn send_error_msg(
    channel_id: ChannelId,
    title: Option<&str>,
    content: &str,
    http: &Http,
) -> Result<(), Box<dyn std::error::Error>> {
    let colour = Colour::new(0xFF0000);
    send_message_with_embed(colour, channel_id, title, content, http)?;
    Ok(())
}

pub fn send_success_msg(
    channel_id: ChannelId,
    title: Option<&str>,
    content: &str,
    http: &Http,
) -> Result<(), Box<dyn std::error::Error>> {
    let colour = Colour::new(0x00FF00);
    send_message_with_embed(colour, channel_id, title, content, http)?;
    Ok(())
}

pub fn send_info_msg(
    channel_id: ChannelId,
    title: Option<&str>,
    content: &str,
    http: &Http,
) -> Result<(), Box<dyn std::error::Error>> {
    let colour = Colour::new(0x0000FF);
    send_message_with_embed(colour, channel_id, title, content, http)?;
    Ok(())
}
