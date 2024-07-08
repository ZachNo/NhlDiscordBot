use serenity::all::{async_trait, Context, EventHandler, Interaction, InteractionType, Ready};

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
        application_commands(ctx.http).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::Autocomplete => {
                autocomplete_interaction(&ctx, interaction.as_autocomplete()).await;
            }
            InteractionType::Command => {
                application_command_interaction(&ctx, interaction.as_command()).await;
            }
            InteractionType::Component => {
                message_component_interaction(&ctx, interaction.as_message_component()).await;
            }
            _ => {}
        }
    }
}
