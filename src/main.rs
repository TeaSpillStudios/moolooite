mod commands;

use dotenv::dotenv;
use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Recieved command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => String::from("Not yet implemented"),
            };

            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to the interaction: {}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected.", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("GUILD_ID must be defined in the environment.")
                .parse()
                .expect("GUILD_ID must be an integer."),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::register(command))
        })
        .await;

        println!("Following guild slash commands registered: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN must be defined in the environment.");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Failed to create the client.");

    if let Err(e) = client.start().await {
        println!("Error with client: {:?}", e);
    }
}
