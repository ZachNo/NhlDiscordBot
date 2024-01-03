use crate::nhl::fetch_data::{fetch_match_score, fetch_schedule};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serenity::{
    builder::{CreateActionRow, CreateButton, CreateComponents, CreateEmbed},
    model::application::component::ButtonStyle::Secondary,
    utils::Colour,
};

pub async fn pull_todays_schedule() -> Result<CreateEmbed> {
    let schedule = fetch_schedule().await?;
    if schedule.games.is_empty() {
        let mut embed: CreateEmbed = CreateEmbed::default();
        embed.color(Colour::from_rgb(240, 200, 0));
        embed.title("There's no games scheduled for today. :c");
        return Ok(embed);
    }

    let mut embed: CreateEmbed = CreateEmbed::default();
    embed.color(Colour::from_rgb(240, 200, 0));
    embed.title(format!("NHL Games for {}", &schedule.date));
    for game in &schedule.games {
        let datetime = DateTime::parse_from_rfc3339(game.start_time_u_t_c.as_str())?;
        embed.field(
            format!(
                "{} vs. {}",
                game.home_team.place_name.default, game.away_team.place_name.default
            ),
            format!(
                "At {} @ <t:{}:t>\n{}",
                game.venue.default,
                datetime.timestamp().to_string(),
                translate_match_status(game.game_state.to_string()),
            ),
            true,
        );
    }

    Ok(embed)
}

pub async fn pull_match_score(match_id: u64) -> Result<CreateEmbed> {
    let match_data = fetch_match_score(match_id).await?;

    let mut embed: CreateEmbed = CreateEmbed::default();
    embed.color(Colour::from_rgb(240, 200, 0));
    embed.title(format!(
        "{} vs. {}",
        match_data.home_team.name.default, match_data.away_team.name.default
    ));
    embed.field(
        "Status",
        translate_match_status(match_data.game_state.to_string()),
        false,
    );
    embed.field(
        "Score",
        format!(
            "{}-{}",
            match_data.home_team.score.unwrap_or(0).to_string(),
            match_data.away_team.score.unwrap_or(0).to_string(),
        ),
        false,
    );
    embed.field(
        "Shots",
        format!(
            "{}-{}",
            match_data.home_team.sog.unwrap_or(0).to_string(),
            match_data.away_team.sog.unwrap_or(0).to_string(),
        ),
        false,
    );
    embed.field(
        format!("{} Period", match_data.period),
        format!("Time left: {}", match_data.clock.time_remaining),
        false,
    );
    embed.field(
        "-",
        format!(
            "Last refreshed <t:{}:R>",
            Utc::now().timestamp().to_string()
        ),
        false,
    );

    Ok(embed)
}

pub async fn get_score_refresh_button(match_id: u64) -> CreateComponents {
    let mut button = CreateButton::default();
    button.label("Refresh");
    button.style(Secondary);
    button.custom_id(format!("score_{}", match_id.to_string()));
    let mut action_row: CreateActionRow = CreateActionRow::default();
    action_row.add_button(button);
    let mut components = CreateComponents::default();
    components.add_action_row(action_row);
    components
}

fn translate_match_status(game_state: String) -> String {
    match game_state.as_str() {
        "OFF" => "Finished".to_string(),
        "LIVE" => "In Progress".to_string(),
        "FUT" => "Scheduled".to_string(),
        _ => {
            println!("Unknown game state: {game_state}");
            "Unknown".to_string()
        },
    }
}
