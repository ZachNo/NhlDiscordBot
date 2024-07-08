use crate::nhl::fetch_data::{fetch_match_score, fetch_schedule};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serenity::{
    builder::{CreateActionRow, CreateButton, CreateEmbed},
    model::{application::ButtonStyle::Secondary, Colour},
};

pub async fn pull_todays_schedule() -> Result<CreateEmbed> {
    let schedule = fetch_schedule().await?;
    if schedule.games.is_empty() {
        let embed: CreateEmbed = CreateEmbed::default()
            .color(Colour::from_rgb(240, 200, 0))
            .title("There's no games scheduled for today. :c");
        return Ok(embed);
    }

    let mut embed: CreateEmbed = CreateEmbed::default()
        .color(Colour::from_rgb(240, 200, 0))
        .title(format!("NHL Games for {}", &schedule.date));
    for game in &schedule.games {
        let datetime = DateTime::parse_from_rfc3339(game.start_time_u_t_c.as_str())?;
        embed = embed.field(
            format!(
                "{} vs. {}",
                game.home_team.place_name.default, game.away_team.place_name.default
            ),
            format!(
                "At {} @ <t:{}:t>\n{}",
                game.venue.default,
                datetime.timestamp(),
                translate_match_status(&game.game_state),
            ),
            true,
        );
    }

    Ok(embed)
}

pub async fn pull_match_score(match_id: u64) -> Result<CreateEmbed> {
    let match_data = fetch_match_score(match_id).await?;
    let mut embed: CreateEmbed = CreateEmbed::default()
        .color(Colour::from_rgb(240, 200, 0))
        .title(format!(
            "{} vs. {}",
            match_data.home_team.name.default, match_data.away_team.name.default
        ))
        .field(
            "Status",
            translate_match_status(&match_data.game_state),
            false,
        )
        .field(
            "Score",
            format!(
                "{}-{}",
                match_data.home_team.score.unwrap_or(0),
                match_data.away_team.score.unwrap_or(0),
            ),
            false,
        )
        .field(
            "Shots",
            format!(
                "{}-{}",
                match_data.home_team.sog.unwrap_or(0),
                match_data.away_team.sog.unwrap_or(0),
            ),
            false,
        );

    if match_data.period.is_some() {
        embed = embed.field(
            format!("{} Period", match_data.period.unwrap()),
            format!("Time left: {}", match_data.clock.time_remaining),
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

pub async fn get_score_refresh_button(match_id: u64) -> CreateActionRow {
    let button = CreateButton::new(format!("score_{match_id}"))
        .label("Refresh")
        .style(Secondary);
    CreateActionRow::Buttons(vec![button])
}

fn translate_match_status<'a>(game_state: &String) -> &'a str {
    match game_state.as_str() {
        "OFF" => "Finished",
        "FINAL" => "Finished",
        "LIVE" => "In Progress",
        "PRE" => "Pre-game",
        "FUT" => "Scheduled",
        _ => {
            println!("Unknown game state: {game_state}");
            "Unknown"
        }
    }
}
