//add commands here
mod say;
use say::SAY_COMMAND;

use serenity::framework::standard::macros::group;

//commands allowed to admins
group!({
    name: "admin",
    options: {
        allowed_roles: ["admin"]
    },
    commands: [say],
});
