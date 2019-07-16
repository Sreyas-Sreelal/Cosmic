//add commands here
mod cmd_say;

use crate::command::cmd_say::*;

use serenity::framework::standard::macros::group;

//commands allowed to admins
group!({
    name: "admin",
    options: {
        allowed_roles: ["admin"]
    },
    commands: [say],
});

//
//group!({
//   name: "general",
//    commands: [say],
//});
//
