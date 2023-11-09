use anyhow::{anyhow, Error, Result};
use serde_json::Value;
use serenity::{
    builder::{CreateComponents, CreateEmbed},
    model::application::interaction::{
        application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction,
        message_component::MessageComponentInteraction, InteractionResponseType,
    },
    prelude::Context,
};

use crate::{
    discord::helpers::get_match_id,
    nhl::{
        autocomplete::populate_match_autocomplete,
        commands::{get_score_refresh_button, pull_match_score, pull_todays_schedule},
    },
};

pub async fn application_command_interaction(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
    let match_id = get_match_id(&command.data).await.unwrap_or(0);

    let embed: CreateEmbed = match command.data.name.as_str() {
        "schedule" => pull_todays_schedule().await?,
        "score" => pull_match_score(match_id.clone()).await?,
        _ => {
            let mut embed = CreateEmbed::default();
            embed.0.insert("type", Value::String("not".to_string()));
            embed
        }
    };

    let components: CreateComponents = match command.data.name.as_str() {
        "score" => get_score_refresh_button(match_id.clone()).await,
        _ => CreateComponents::default(),
    };

    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    if embed.0["type"] == "rich" {
                        message.add_embed(embed).set_components(components);
                    }
                    message
                })
        })
        .await
        .map_err(Error::msg)
}

pub async fn autocomplete_interaction(
    ctx: &Context,
    autocomplete: &AutocompleteInteraction,
) -> Result<()> {
    match autocomplete.data.name.as_str() {
        "score" => autocomplete_interaction_score(ctx, autocomplete).await?,
        _ => {}
    }
    Ok(())
}

async fn autocomplete_interaction_score(
    ctx: &Context,
    autocomplete: &AutocompleteInteraction,
) -> Result<()> {
    let user_input = get_user_input(autocomplete, "match".to_string())?;
    let matches = populate_match_autocomplete(
        user_input
            .strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap()
            .to_string(),
    )
    .await?;
    autocomplete
        .create_autocomplete_response(&ctx.http, |response| {
            for (title, id) in matches {
                response.add_string_choice(title, id);
            }
            response
        })
        .await
        .map_err(Error::msg)
}

pub async fn message_component_interaction(
    ctx: &Context,
    message: &MessageComponentInteraction,
) -> Result<()> {
    let match_id = message
        .data
        .custom_id
        .strip_prefix("score_")
        .unwrap()
        .parse::<u64>()?;
    let new_message = pull_match_score(match_id.clone()).await?;
    let new_components = get_score_refresh_button(match_id).await;

    message
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::UpdateMessage);
            response.interaction_response_data(|response_data| {
                response_data
                    .add_embed(new_message)
                    .set_components(new_components)
            })
        })
        .await
        .map_err(Error::msg)
}

fn get_user_input(autocomplete: &AutocompleteInteraction, name: String) -> Result<String> {
    Ok(autocomplete
        .data
        .options
        .iter()
        .find(|x| x.name == name)
        .ok_or(anyhow!("Cannot find name"))?
        .value
        .as_ref()
        .unwrap()
        .to_string())
}
