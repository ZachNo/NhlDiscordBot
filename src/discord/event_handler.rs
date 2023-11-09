use serenity::{
    async_trait,
    model::{
        application::{command::Command, interaction::Interaction},
        gateway::Ready,
        prelude::interaction::InteractionType,
    },
    prelude::{Context, EventHandler},
};

use crate::discord::commands::application_commands;
use crate::discord::interactions::{
    application_command_interaction, autocomplete_interaction, message_component_interaction,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        println!("Total servers in: {}", ready.guilds.len());
        let _ = Command::set_global_application_commands(&ctx.http, application_commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::ApplicationCommand => {
                application_command_interaction(&ctx, &interaction.application_command().unwrap())
                    .await
                    .unwrap()
            }
            InteractionType::Autocomplete => {
                autocomplete_interaction(&ctx, &interaction.autocomplete().unwrap())
                    .await
                    .unwrap()
            }
            InteractionType::MessageComponent => {
                message_component_interaction(&ctx, &interaction.message_component().unwrap())
                    .await
                    .unwrap()
            }
            _ => {}
        }
    }
}
