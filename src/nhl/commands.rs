use chrono::{
    DateTime,
    Utc,
};
use serenity::{
    builder::{
        CreateActionRow,
        CreateButton,
        CreateComponents,
        CreateEmbed,
    },
    model::application::component::ButtonStyle::Secondary,
    utils::Colour,
};
use crate::nhl::fetch_data::{
    fetch_match_highlights,
    fetch_match_score,
    fetch_schedule_from_web,
};

pub async fn pull_todays_schedule() -> CreateEmbed {
    let schedule = fetch_schedule_from_web().await;
    if schedule.dates.is_empty() {
        let mut embed: CreateEmbed = CreateEmbed::default();
        embed.color(Colour::from_rgb(240, 200, 0));
        embed.title("There's no games scheduled for today. :c");
        return embed
    }
    let today = schedule.dates.get(0).unwrap();

    let mut embed: CreateEmbed = CreateEmbed::default();
    embed.color(Colour::from_rgb(240, 200, 0));
    embed.title(format!("NHL Games for {}", &today.date));
    for game in &today.games {
        let datetime = DateTime::parse_from_rfc3339(game.game_date.as_str()).unwrap();
        embed.field(
            format!("{} vs. {}",
                    game.teams.home.team.name,
                    game.teams.away.team.name
            ),
            format!("At {} @ <t:{}:t>\n{}",
                    game.venue.name,
                    datetime.timestamp().to_string(),
                    game.status.detailed_state,
            ),
            true,
        );
    }

    embed
}

pub async fn pull_match_score(match_id: u64) -> CreateEmbed {
    let match_data = fetch_match_score(match_id).await;

    let mut embed: CreateEmbed = CreateEmbed::default();
    embed.color(Colour::from_rgb(240, 200, 0));
    embed.title(format!("{} vs. {}",
                        match_data.live_data.boxscore.teams.home.team.name,
                        match_data.live_data.boxscore.teams.away.team.name
    ));
    embed.field(
        "Status",
        match_data.game_data.status.detailed_state,
        false,
    );
    embed.field(
        "Score",
        format!("{}-{}",
                match_data.live_data.boxscore.teams.home.team_stats.team_skater_stats.goals.to_string(),
                match_data.live_data.boxscore.teams.away.team_stats.team_skater_stats.goals.to_string(),
        ),
        false,
    );
    embed.field(
        "Shots",
        format!("{}-{}",
                match_data.live_data.boxscore.teams.home.team_stats.team_skater_stats.shots.to_string(),
                match_data.live_data.boxscore.teams.away.team_stats.team_skater_stats.shots.to_string(),
        ),
        false,
    );
    embed.field(
        format!("{} Period", match_data.live_data.linescore.current_period_ordinal),
        format!("Time left: {}", match_data.live_data.linescore.current_period_time_remaining),
        false,
    );
    embed.field(
        "-",
        format!("Last refreshed <t:{}:R>", Utc::now().timestamp().to_string()),
        false,
    );

    embed
}

pub async fn pull_match_highlight(match_id: u64, highlight_id: u64) -> String {
    let highlight_data = fetch_match_highlights(match_id).await;
    let specific_highlight = highlight_data.highlights.game_center.items
        .iter().find(|x| x.id.parse::<u64>().unwrap() == highlight_id).unwrap();
    let highlight_link = specific_highlight.playbacks
        .iter().find(|x| x.name == "FLASH_1800K_896x504").unwrap();

    format!("**{}**\n{}", specific_highlight.title, highlight_link.url)
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