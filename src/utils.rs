use lazy_static::lazy_static;
use regex::Regex;

//removes mentions from the message
pub fn remove_mention(msg: &str) -> String {
    lazy_static! {
        static ref MENTION_RE: Regex = Regex::new("<@[0-9]+>").unwrap();
    }
    MENTION_RE.replace_all(&msg, "").to_string()
}
