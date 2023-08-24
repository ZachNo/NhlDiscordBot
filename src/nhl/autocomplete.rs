use crate::nhl::fetch_data::{
    fetch_match_highlights,
    fetch_schedule_from_web,
};

pub async fn populate_match_autocomplete(user_input: String) -> Vec<(String, u64)> {
    let schedule = fetch_schedule_from_web().await;

    let mut matches: Vec<(String, u64)> = Vec::new();
    for date in &schedule.dates {
        for game in &date.games {
            let title = format!(
                "{} vs. {}",
                game.teams.home.team.name,
                game.teams.away.team.name
            );
            if user_input.is_empty() || title.to_lowercase().contains(user_input.as_str()) {
                matches.push((
                    title,
                    game.game_pk.clone()
                ));
            }
        }
    }
    matches
}

pub async fn populate_highlight_autocomplete(match_id: u64, user_input: String) -> Vec<(String, u64)> {
    let highlight_data = fetch_match_highlights(match_id).await;

    let mut matches: Vec<(String, u64)> = Vec::new();
    for highlight in highlight_data.highlights.game_center.items {
        if user_input.is_empty() || highlight.title.to_lowercase().contains(user_input.as_str()) {
            matches.push((
                highlight.title,
                highlight.id.parse::<u64>().unwrap(),
            ));
        }
    }
    matches
}