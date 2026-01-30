use poise::serenity_prelude::{ChannelId, Context, GuildId, UserId};
use songbird::{Songbird, get};


// TODO: Make the bot say that it is already in the voice channel if it is already connected.
pub async fn join_voice_channel(
    serenity_ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
) -> Result<ChannelId, String> {
    let channel_id: ChannelId = {
        let guild = guild_id
            .to_guild_cached(&serenity_ctx.cache)
            .ok_or_else(|| "Guild not cached yet. Try again later.".to_string())?;

        guild
            .voice_states
            .get(&user_id)
            .and_then(|vs| vs.channel_id)
            .ok_or_else(|| "User is not in a voice channel.".to_string())?
    };

    let manager: std::sync::Arc<Songbird> = get(serenity_ctx)
        .await
        .ok_or_else(|| "Songbird voice client not initialized.".to_string())?;
    
    manager
        .join(guild_id, channel_id)
        .await
        .map_err(|e| format!("Failed to join voice channel: {}", e))?;

    Ok(channel_id)
}

pub async fn leave_voice_channel(serenity_ctx: &Context, guild_id: GuildId) -> Result<(), String> {
    let manager = get(serenity_ctx)
        .await
        .ok_or_else(|| "Songbird voice client not initialized.".to_string())?;

    if manager.get(guild_id).is_none() {
        return Err("Not in a voice channel.".to_string());
    }

    manager
        .leave(guild_id)
        .await
        .map_err(|e| format!("Failed to leave voice channel: {}", e))?;

    Ok(())
}
