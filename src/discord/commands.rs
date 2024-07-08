use anyhow::Result;
use serenity::all::{Command, CommandOptionType, CreateCommand, CreateCommandOption, Http};
use std::sync::Arc;

pub async fn application_commands(http: Arc<Http>) -> Result<()> {
    Command::create_global_command(
        &http,
        CreateCommand::new("schedule").description("Print the schedule for today's games"),
    )
    .await?;
    Command::create_global_command(
        &http,
        CreateCommand::new("score")
            .description("Print the live score for a match")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "match",
                    "Which match to view the score of.",
                )
                .required(true)
                .set_autocomplete(true),
            ),
    )
    .await?;

    Ok(())
}
