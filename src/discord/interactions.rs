use serenity::all::{
    CommandInteraction, ComponentInteraction, Context, CreateAutocompleteResponse,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::discord::commands::DiscordCommand;
use crate::error::error_to_error_message;
use crate::error::DiscordError::InvalidCommand;

pub async fn application_command_interaction(
    ctx: &Context,
    command_opt: Option<&CommandInteraction>,
    err_chan: Option<u64>,
) {
    if let Some(command) = command_opt {
        let discord_command = match DiscordCommand::try_from(command.data.name.as_str()) {
            Ok(d) => d,
            Err(_) => {
                command
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .ephemeral(true)
                                .content(
                                    error_to_error_message(
                                        InvalidCommand(command.data.name.clone()).into(),
                                        ctx,
                                        err_chan,
                                    )
                                    .await,
                                ),
                        ),
                    )
                    .await
                    .unwrap();
                return;
            }
        }
        .into_command();
        let (embed, components) = match discord_command.handle_command(command).await {
            Ok((e, c)) => (e, c),
            Err(e) => {
                command
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .ephemeral(true)
                                .content(error_to_error_message(e, ctx, err_chan).await),
                        ),
                    )
                    .await
                    .unwrap();
                return;
            }
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
    err_chan: Option<u64>,
) {
    if let Some(autocomplete) = autocomplete_opt {
        let discord_command = match DiscordCommand::try_from(autocomplete.data.name.as_str()) {
            Ok(d) => d,
            Err(_) => {
                autocomplete
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .ephemeral(true)
                                .content(
                                    error_to_error_message(
                                        InvalidCommand(autocomplete.data.name.clone()).into(),
                                        ctx,
                                        err_chan,
                                    )
                                    .await,
                                ),
                        ),
                    )
                    .await
                    .unwrap();
                return;
            }
        }
        .into_command();
        let response_options = match discord_command.handle_autocomplete(autocomplete).await {
            Ok(r) => r,
            Err(e) => {
                error_to_error_message(e, ctx, err_chan).await;
                CreateAutocompleteResponse::new()
            }
        };

        autocomplete
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Autocomplete(response_options),
            )
            .await
            .unwrap();
    }
}

pub async fn message_component_interaction(
    ctx: &Context,
    message_opt: Option<&ComponentInteraction>,
    err_chan: Option<u64>,
) {
    if let Some(message) = message_opt {
        let command_str = message.data.custom_id.split("_").collect::<Vec<_>>()[0];
        let discord_command = match DiscordCommand::try_from(command_str) {
            Ok(d) => d,
            Err(_) => {
                message
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .ephemeral(true)
                                .content(
                                    error_to_error_message(
                                        InvalidCommand(command_str.to_string()).into(),
                                        ctx,
                                        err_chan,
                                    )
                                    .await,
                                ),
                        ),
                    )
                    .await
                    .unwrap();
                return;
            }
        }
        .into_command();
        let (embed, components) = match discord_command.handle_interaction(message).await {
            Ok((e, c)) => (e, c),
            Err(e) => {
                message
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .ephemeral(true)
                                .content(error_to_error_message(e, ctx, err_chan).await),
                        ),
                    )
                    .await
                    .unwrap();
                return;
            }
        };

        message
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(embed)
                        .components(components),
                ),
            )
            .await
            .unwrap();
    }
}
