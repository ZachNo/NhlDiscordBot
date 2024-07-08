use anyhow::{anyhow, Context as AnyhowContext, Result};
use serenity::all::{
    CommandInteraction, ComponentInteraction, Context, CreateAutocompleteResponse, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::error::{error_to_error_message, NON_USER_ERROR_OUTPUT};
use crate::{
    discord::helpers::get_match_id,
    nhl::{
        autocomplete::populate_match_autocomplete,
        commands::{get_score_refresh_button, pull_match_score, pull_todays_schedule},
    },
};

pub async fn application_command_interaction(
    ctx: &Context,
    command_opt: Option<&CommandInteraction>,
) {
    if let Some(command) = command_opt {
        let (embed, components) = match command.data.name.as_str() {
            "schedule" => {
                let schedule = match pull_todays_schedule().await {
                    Ok(s) => s,
                    Err(e) => {
                        command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .ephemeral(true)
                                        .content(error_to_error_message(e)),
                                ),
                            )
                            .await
                            .unwrap();
                        return;
                    }
                };
                (schedule, vec![])
            }
            "score" => {
                let match_id = match get_match_id(&command.data).await {
                    Ok(i) => i,
                    Err(e) => {
                        command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .ephemeral(true)
                                        .content(error_to_error_message(e)),
                                ),
                            )
                            .await
                            .unwrap();
                        return;
                    }
                };
                let score = match pull_match_score(match_id).await {
                    Ok(s) => s,
                    Err(e) => {
                        command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .ephemeral(true)
                                        .content(error_to_error_message(e)),
                                ),
                            )
                            .await
                            .unwrap();
                        return;
                    }
                };
                (score, vec![get_score_refresh_button(match_id).await])
            }
            _ => (CreateEmbed::default(), vec![]),
        };

        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(embed)
                        .components(components),
                ),
            )
            .await
            .unwrap();
    }
}

pub async fn autocomplete_interaction(
    ctx: &Context,
    autocomplete_opt: Option<&CommandInteraction>,
) {
    if let Some(autocomplete) = autocomplete_opt {
        if autocomplete.data.name.as_str() == "score" {
            match autocomplete_interaction_score(ctx, autocomplete).await {
                Ok(_) => {}
                Err(e) => {
                    autocomplete
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .ephemeral(true)
                                    .content(error_to_error_message(e)),
                            ),
                        )
                        .await
                        .unwrap();
                }
            }
        }
    }
}

async fn autocomplete_interaction_score(
    ctx: &Context,
    autocomplete: &CommandInteraction,
) -> Result<()> {
    let user_input = get_user_input(autocomplete, "match".to_string())?;
    let matches = populate_match_autocomplete(user_input).await?;
    let mut response_options = CreateAutocompleteResponse::new();
    for (title, id) in matches {
        response_options = response_options.add_string_choice(title, id.to_string());
    }

    autocomplete
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Autocomplete(response_options),
        )
        .await
        .context("Failed to create response")
}

pub async fn message_component_interaction(
    ctx: &Context,
    message_opt: Option<&ComponentInteraction>,
) {
    if let Some(message) = message_opt {
        let match_id_str = match message.data.custom_id.strip_prefix("slapshot_score_") {
            Some(s) => s,
            None => {
                println!("Error: weird message id update: {}", message.data.custom_id);
                message
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .ephemeral(true)
                                .content(NON_USER_ERROR_OUTPUT.to_string()),
                        ),
                    )
                    .await
                    .unwrap();
                return;
            }
        };
        let match_id = match match_id_str.parse::<u64>() {
            Ok(u) => u,
            Err(e) => {
                println!("{e}");
                message
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .ephemeral(true)
                                .content(NON_USER_ERROR_OUTPUT.to_string()),
                        ),
                    )
                    .await
                    .unwrap();
                return;
            }
        };
        let new_message = match pull_match_score(match_id).await {
            Ok(m) => m,
            Err(e) => {
                message
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .ephemeral(true)
                                .content(error_to_error_message(e)),
                        ),
                    )
                    .await
                    .unwrap();
                return;
            }
        };
        let new_components = get_score_refresh_button(match_id).await;

        message
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(new_message)
                        .components(vec![new_components]),
                ),
            )
            .await
            .unwrap();
    }
}

fn get_user_input(autocomplete: &CommandInteraction, name: String) -> Result<String> {
    Ok(autocomplete
        .data
        .options
        .iter()
        .find(|x| x.name == name)
        .ok_or(anyhow!("Cannot find name"))?
        .value
        .as_str()
        .ok_or(anyhow!("Unable to grab string"))?
        .to_string())
}
