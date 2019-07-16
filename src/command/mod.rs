//add commands here
mod cmd_meme;
mod cmd_say;
mod cmd_wiki;

use crate::command::{cmd_meme::*, cmd_say::*, cmd_wiki::*};

use serenity::framework::standard::macros::group;

//commands allowed to admins
group!({
    name: "admin",
    options: {
        allowed_roles: ["admin"]
    },
    commands: [say],
});

group!({
    name: "general",
    commands: [meme,wiki],
});
