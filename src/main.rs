mod commands;

use std::env;

use dotenvy::dotenv;
use serenity::async_trait;

use serenity::{
    client::{Client, Context, EventHandler},
    framework::standard::CommandResult,
    model::{channel::Message, gateway::Ready, prelude::*},
    prelude::GatewayIntents,
};

use songbird::SerenityInit;

use crate::commands::math::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
            println!("Received a ping command from {}", msg.author.name);
        }
        if msg.content.starts_with("!math") {
            let mut input = msg.content.split_whitespace().collect::<Vec<_>>();
            input.remove(0); // Remove !math
            let response = parse_math_command(input)
                .await
                .unwrap_or_else(|e| format!("Error: {}", e));
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "!join" {
            if let Err(why) = join_voice(&ctx, &msg).await {
                println!("Error joining voice channel: {:?}", why);
            }
        }
    }

    // async fn guild_member_addition(
    //     &self,
    //     ctx: Context,
    //     new_member: serenity::model::guild::Member,
    // ) {
    //     let welcome_message = format!("Welcome to the server, {}!", new_member.user.name);
    //     if let Err(why) = guild_id
    //         .system_channel(&ctx.http)
    //         .await
    //         .unwrap()
    //         .say(&ctx.http, welcome_message)
    //         .await
    //     {
    //         println!("Error sending welcome message: {:?}", why);
    //     }
    // }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn join_voice(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(guild_id) => guild_id,
        None => {
            msg.reply(ctx, "This command can only be used in a server")
                .await?;
            return Ok(());
        }
    };

    let channel_id_opt: Option<ChannelId> = guild_id.to_guild_cached(&ctx.cache).and_then(|g| {
        g.voice_states
            .get(&msg.author.id)
            .and_then(|vs| vs.channel_id)
    });

    let channel_id = match channel_id_opt {
        Some(c) => c,
        None => {
            msg.reply(
                ctx,
                "You need to be in a voice channel to use this command.",
            )
            .await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    match manager.join(guild_id, channel_id).await {
        Ok(_handler_lock) => {
            msg.reply(
                ctx,
                format!("Joined voice channel: {}", channel_id.mention()),
            )
            .await?;
        }
        Err(why) => {
            msg.reply(ctx, format!("Error joining voice channel: {}", why))
                .await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok().expect("Could not load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
