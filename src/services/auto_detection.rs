use crate::framework::prelude::Context;
use std::error::Error;
use twilight_model::id::{GuildId, UserId};
use tokio::time::{interval, Duration};
use tracing::debug;

pub async fn auto_detection(ctx: Context) {
    let mut interval = interval(Duration::from_secs(3 * 3600));
    std::thread::sleep(Duration::from_secs(15));
    loop {
        let _ = execute(&ctx).await;
        interval.tick().await;
    }
}

async fn execute(ctx: &Context) -> Result<(), Box<dyn Error>> {
    let servers = ctx.cache.guilds();
    let mut guilds = ctx.database.get_guilds(servers, true).await?;
    guilds.sort_by_key(|g| g.id);
    for guild in guilds {
        let start = chrono::Utc::now().timestamp_millis();
        let guild_id = GuildId(guild.id as u64);
        let server = match ctx.cache.guild(guild_id) {
            Some(g) => g,
            None => continue
        };
        let members = ctx.cache.members(guild_id).into_iter().map(|m| m.0).collect::<Vec<_>>();
        let users = ctx.database.get_users(members).await?;
        println!("{:?}", users.len());
        let bypass = ctx.cache.bypass_roles(guild_id);
        let guild_roles = ctx.cache.roles(guild_id);
        for user in users {
            if let Some(member) = ctx.cache.member(guild_id, UserId(user.discord_id as u64)) {
                if let Some(bypass) = bypass.0 {
                    if member.roles.contains(&bypass) {continue;}
                }
                debug!(id = user.discord_id, "Auto Detection for member");
                if let Err(e) = user.update(ctx.http.clone(), member, ctx.roblox.clone(), server.clone(), &guild, &guild_roles).await {
                    println!("{:?}", e);
                }
            }
        }
        let end = chrono::Utc::now().timestamp_millis();
        println!("Time to complete auto detection: {:?}", end - start);
    }
    Ok(())
}