mod commands;

use std::env;

use dotenvy::dotenv;

use serenity::{model::prelude::*, prelude::GatewayIntents};

use songbird::SerenityInit;

use crate::commands::math::*;
use crate::commands::voice::*;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct Data;

type PoiseContext<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
async fn ping(ctx: PoiseContext<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn math(ctx: PoiseContext<'_>, a: i32, op: String, b: i32) -> Result<(), Error> {
    let parts = vec![a.to_string(), op, b.to_string()];
    let parts: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();

    match parse_math_command(parts).await {
        Ok(result) => ctx.say(result).await?,
        Err(e) => ctx.say(format!("Error: {}", e)).await?,
    };

    Ok(())
}

#[poise::command(prefix_command, guild_only)]
pub async fn join(ctx: PoiseContext<'_>) -> Result<(), Error> {
    let serenity_ctx = ctx.serenity_context();
    let guild_id = ctx.guild_id().unwrap();
    let user_id = ctx.author().id;

    match join_voice_channel(serenity_ctx, guild_id, user_id).await {
        Ok(channel_id) => {
            ctx.reply(format!("Joined {}!", channel_id.mention()))
                .await?;
        }
        Err(msg) => {
            ctx.reply(msg).await?;
        }
    }
    Ok(())
}

#[poise::command(prefix_command, guild_only)]
pub async fn leave(ctx: PoiseContext<'_>) -> Result<(), Error> {
    let serenity_ctx = ctx.serenity_context();
    let guild_id = ctx.guild_id().unwrap();

    match leave_voice_channel(serenity_ctx, guild_id).await {
        Ok(()) => {
            ctx.reply("Left the channel.").await?;
        }
        Err(msg) => {
            ctx.reply(msg).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok().expect("Could not load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![join(), leave(), ping(), math()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|_ctx, ready, _framework| {
            Box::pin(async move {
                println!("{} is connected!", ready.user.name);
                Ok(Data)
            })
        })
        .build();

    let mut client = poise::serenity_prelude::Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    client.start().await?;
    Ok(())
}
