mod info;
mod test;
mod update;
mod verify;

pub use info::{botinfo, support, userinfo};
use rowifi_framework::prelude::*;
pub use test::test;
pub use update::update;
pub use verify::{reverify, verify};

pub fn user_config(cmds: &mut Vec<Command>) {
    let update_cmd = Command::builder()
        .level(RoLevel::Normal)
        .names(&["update"])
        .description("Command to update an user")
        .group("User")
        .handler(update);

    let userinfo_cmd = Command::builder()
        .level(RoLevel::Normal)
        .names(&["userinfo"])
        .description("Command to view information about an user")
        .group("User")
        .handler(userinfo);

    let botinfo_cmd = Command::builder()
        .level(RoLevel::Normal)
        .names(&["botinfo"])
        .description("Command to view information about the bot")
        .group("User")
        .handler(botinfo);

    let support_cmd = Command::builder()
        .level(RoLevel::Normal)
        .names(&["support", "invite"])
        .description("View important links related to the bot")
        .group("User")
        .handler(support);

    let verify_cmd = Command::builder()
        .level(RoLevel::Normal)
        .names(&["verify"])
        .description("Command to link a roblox account to your discord account")
        .group("User")
        .handler(verify);

    let reverify_cmd = Command::builder()
        .level(RoLevel::Normal)
        .names(&["reverify"])
        .description("Command to re-link a roblox account to your discord account")
        .group("User")
        .handler(reverify);

    let test_cmd = Command::builder()
        .level(RoLevel::Creator)
        .names(&["test"])
        .handler(test);

    cmds.push(update_cmd);
    cmds.push(userinfo_cmd);
    cmds.push(botinfo_cmd);
    cmds.push(support_cmd);
    cmds.push(verify_cmd);
    cmds.push(reverify_cmd);
    cmds.push(test_cmd);
}
