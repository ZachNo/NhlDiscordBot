use serenity::all::{
    async_trait, Command, Context, EventHandler, Interaction, InteractionType, Ready,
};
use strum::IntoEnumIterator;

use crate::discord::commands::DiscordCommand;
use crate::discord::config::Config;
use crate::discord::influx::send_usage_metric;
use crate::discord::interactions::{
    application_command_interaction, autocomplete_interaction, message_component_interaction,
};

pub struct Handler {
    pub(crate) config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        println!("Total servers in: {}", ready.guilds.len());
        for command_enum in DiscordCommand::iter() {
            let create_command = command_enum.into_command().create_command();
            Command::create_global_command(&ctx.http, create_command)
                .await
                .unwrap();
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::Autocomplete => {
                let autocomplete = interaction.as_autocomplete();
                if autocomplete.is_some() {
                    send_usage_metric(
                        &self.config,
                        "autocomplete".to_string(),
                        autocomplete
                            .and_then(|a| Some(a.data.name.clone()))
                            .unwrap(),
                    )
                    .await;
                }
                autocomplete_interaction(&ctx, autocomplete, self.config.error_channel).await;
            }
            InteractionType::Command => {
                let command = interaction.as_command();
                if command.is_some() {
                    send_usage_metric(
                        &self.config,
                        "command".to_string(),
                        command.and_then(|c| Some(c.data.name.clone())).unwrap(),
                    )
                    .await;
                }
                application_command_interaction(&ctx, command, self.config.error_channel).await;
            }
            InteractionType::Component => {
                let message_component = interaction.as_message_component();
                if message_component.is_some() {
                    send_usage_metric(
                        &self.config,
                        "component".to_string(),
                        message_component
                            .and_then(|c| Some(c.data.custom_id.clone()))
                            .unwrap(),
                    )
                    .await;
                }
                message_component_interaction(&ctx, message_component, self.config.error_channel)
                    .await;
            }
            _ => {}
        }
    }
}
