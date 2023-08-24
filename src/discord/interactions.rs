use serenity::{
    builder::{
        CreateComponents,
        CreateEmbed,
    },
    model::application::interaction::{
        application_command::ApplicationCommandInteraction,
        autocomplete::AutocompleteInteraction,
        InteractionResponseType,
        message_component::MessageComponentInteraction,
    },
    prelude::Context,
};
use serde_json::Value;

use crate::{
    discord::helpers::{
        get_match_id,
        get_highlight_id_from_input,
    },
    nhl::{
        autocomplete::{
            populate_highlight_autocomplete,
            populate_match_autocomplete,
        },
        commands::{
            get_score_refresh_button,
            pull_match_highlight,
            pull_match_score,
            pull_todays_schedule,
        },
    }
};


pub async fn application_command_interaction(ctx: &Context, command: &ApplicationCommandInteraction) {
    let content: String = match command.data.name.as_str() {
        "highlight" => pull_match_highlight(
            get_match_id(&command.data).await,
            get_highlight_id_from_input(&command).await,
        ).await,
        _ => "".to_string()
    };

    let embed: CreateEmbed = match command.data.name.as_str() {
        "schedule" => pull_todays_schedule().await,
        "score" => pull_match_score(get_match_id(&command.data).await).await,
        _ => {
            let mut embed = CreateEmbed::default();
            embed.0.insert("type", Value::String("not".to_string()));
            embed
        }
    };

    let components: CreateComponents = match command.data.name.as_str() {
        "score" => {
            get_score_refresh_button(get_match_id(&command.data).await).await
        }
        _ => CreateComponents::default()
    };

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content(content);
                    if embed.0["type"] == "rich" {
                        message.add_embed(embed).set_components(components);
                    }
                    message
                })
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}

pub async fn autocomplete_interaction(ctx: &Context, autocomplete: &AutocompleteInteraction) {
    match autocomplete.data.name.as_str() {
        "score" => autocomplete_interaction_score(ctx, autocomplete).await,
        "highlight" => autocomplete_interaction_highlight(ctx, autocomplete).await,
        _ => {},
    }
}

async fn autocomplete_interaction_score(ctx: &Context, autocomplete: &AutocompleteInteraction) {
    let user_input = get_user_input(autocomplete, "match".to_string());
    let matches = populate_match_autocomplete(user_input).await;
    if let Err(why) = autocomplete
        .create_autocomplete_response(&ctx.http, |response| {
            for (title, id) in matches {
                response.add_string_choice(title, id);
            }
            response
        })
        .await
    {
        println!("Cannot respond to autocomplete: {}", why);
    }
}

async fn autocomplete_interaction_highlight(ctx: &Context, autocomplete: &AutocompleteInteraction) {
    let focused = autocomplete.data.options
        .iter().find(|x| x.focused).unwrap();
    let matches = match focused.name.as_str() {
        "match" => {
            let user_input = get_user_input(autocomplete, "match".to_string());
            populate_match_autocomplete(user_input).await
        }
        "highlight" => {
            let match_id = get_match_id(&autocomplete.data).await;
            let user_input = get_user_input(autocomplete, "highlight".to_string());
            populate_highlight_autocomplete(match_id, user_input).await
        }
        _ => vec![]
    };
    if let Err(why) = autocomplete
        .create_autocomplete_response(&ctx.http, |response| {
            for (title, id) in matches {
                response.add_string_choice(title, id);
            }
            response
        })
        .await
    {
        println!("Cannot respond to autocomplete: {}", why);
    }
}

pub async fn message_component_interaction(ctx: &Context, message: &MessageComponentInteraction) {
    if message.data.custom_id.starts_with("score_") {
        let match_id = message.data.custom_id.strip_prefix("score_").unwrap().parse::<u64>().unwrap();
        let new_message = pull_match_score(match_id).await;
        let new_components = get_score_refresh_button(match_id).await;

        if let Err(why) = message
            .create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::UpdateMessage);
                response.interaction_response_data(|response_data| {
                    response_data.add_embed(new_message).set_components(new_components)
                })
            })
            .await
        {
            println!("Cannot respond to interaction: {}", why);
        }
    }
}

fn get_user_input(autocomplete: &AutocompleteInteraction, name: String) -> String {
    autocomplete.data.options
        .iter().find(|x| x.name == name).unwrap()
        .value.as_ref().unwrap()
        .to_string()
}