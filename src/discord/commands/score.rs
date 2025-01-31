use anyhow::{anyhow, Result};
use chrono::Utc;
use serenity::all::{
    async_trait, ButtonStyle::Secondary, Colour, CommandData, CommandInteraction,
    CommandOptionType, ComponentInteraction, CreateActionRow, CreateAutocompleteResponse,
    CreateButton, CreateCommand, CreateCommandOption, CreateEmbed,
};

use crate::discord::commands::common::DiscordCommandTrait;
use crate::error::DiscordError::{NoMatchFound, NoValueProvided};
use crate::nhl::fetch_data::{fetch_match_score, fetch_today_schedule};
use crate::nhl::utils::translate_match_status;

pub const NAME: &str = "score";
const DESCRIPTION: &str = "Print the live score for a match";

pub struct Score {}

#[async_trait]
impl DiscordCommandTrait for Score {
    fn create_command(&self) -> CreateCommand {
        CreateCommand::new(NAME)
            .description(DESCRIPTION)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "match",
                    "Which match to view the score of.",
                )
                .required(true)
                .set_autocomplete(true),
            )
    }

    async fn handle_command(
        &self,
        command: &CommandInteraction,
    ) -> Result<(CreateEmbed, Vec<CreateActionRow>)> {
        let match_id = get_match_id_u64(&command.data)?;
        let score = pull_match_score(match_id).await?;
        Ok((score, vec![get_score_refresh_button(match_id)]))
    }

    async fn handle_autocomplete(
        &self,
        autocomplete: &CommandInteraction,
    ) -> Result<CreateAutocompleteResponse> {
        let user_input = get_match_id_str(&autocomplete.data)?;
        let matches = populate_match_autocomplete(user_input.to_string()).await?;
        let mut response_options = CreateAutocompleteResponse::new();
        for (title, id) in matches {
            response_options = response_options.add_string_choice(title, id.to_string());
        }
        Ok(response_options)
    }

    async fn handle_interaction(
        &self,
        message: &ComponentInteraction,
    ) -> Result<(CreateEmbed, Vec<CreateActionRow>)> {
        let match_id_str = message
            .data
            .custom_id
            .strip_prefix("score_")
            .ok_or(anyhow!("malformed custom_id {}", message.data.custom_id))?;
        let match_id = match_id_str.parse::<u64>()?;
        let new_message = pull_match_score(match_id).await?;
        let new_components = get_score_refresh_button(match_id);
        Ok((new_message, vec![new_components]))
    }
}

async fn populate_match_autocomplete(user_input: String) -> Result<Vec<(String, u64)>> {
    let all_matches = populate_matches().await?;
    let mut user_matches = Vec::new();
    for (game, id) in all_matches {
        if user_input.is_empty() || game.to_lowercase().contains(user_input.as_str()) {
            user_matches.push((game, id));
        }
    }
    Ok(user_matches)
}

async fn populate_matches() -> Result<Vec<(String, u64)>> {
    let schedule = fetch_today_schedule().await?;

    let mut matches = Vec::new();
    for game in &schedule.games {
        let title = format!("{} vs. {}", game.get_home_team_full_name(), game.get_away_team_full_name());
        matches.push((title, game.id));
    }
    Ok(matches)
}

async fn pull_match_score(match_id: u64) -> Result<CreateEmbed> {
    let game = fetch_match_score(match_id).await?;
    let mut embed: CreateEmbed = CreateEmbed::default()
        .color(Colour::from_rgb(240, 200, 0))
        .title(format!("{} vs. {}", game.get_home_team_full_name(), game.get_away_team_full_name()))
        .field(
            "Status",
            translate_match_status(&game.game_state),
            false,
        )
        .field(
            "Score",
            format!(
                "{}-{}",
                game.home_team.score.unwrap_or(0),
                game.away_team.score.unwrap_or(0),
            ),
            false,
        )
        .field(
            "Shots",
            format!(
                "{}-{}",
                game.home_team.sog.unwrap_or(0),
                game.away_team.sog.unwrap_or(0),
            ),
            false,
        );

    if game.period_descriptor.is_some() && game.clock.is_some() {
        embed = embed.field(
            format!("Period {}", game.period_descriptor.unwrap().number),
            format!("Time left: {}", game.clock.unwrap().time_remaining),
            false,
        );
    }

    embed = embed.field(
        "-",
        format!("Last refreshed <t:{}:R>", Utc::now().timestamp()),
        false,
    );

    Ok(embed)
}

fn get_score_refresh_button(match_id: u64) -> CreateActionRow {
    let button = CreateButton::new(format!("score_{match_id}"))
        .label("Refresh")
        .style(Secondary);
    CreateActionRow::Buttons(vec![button])
}

fn get_match_id_u64(data: &CommandData) -> Result<u64> {
    let id = get_match_id_str(data)?;
    id.parse::<u64>()
        .map_err(|_| NoMatchFound(id.to_string()).into())
}

fn get_match_id_str(data: &CommandData) -> Result<&str> {
    Ok(data
        .options
        .iter()
        .find(|x| x.name == "match")
        .ok_or(anyhow!("Cannot find match"))?
        .value
        .as_str()
        .ok_or(NoValueProvided)?)
}
