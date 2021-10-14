use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::Interaction;
use serenity::model::interactions::InteractionResponseType;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;

use std::env;

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let gid: u64 = env::var("DISCORD_DEV_GUILD")
            .ok()
            .unwrap()
            .parse::<u64>()
            .expect("Guild ID is not a valid Guild ID!");

        let guild_commands = GuildId(gid)
            .create_application_command(&ctx.http, |command| {
                command
                    .name("ping")
                    .description("Returns the Network Latency!")
            })
            .await;

        println!("Registered Guild Commands: {:#?}", guild_commands)
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content: String = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                _ => "not implemented :(".to_string(),
            };

            if content == "" {}

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        };
    }
}
