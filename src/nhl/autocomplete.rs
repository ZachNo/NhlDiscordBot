use crate::nhl::fetch_data::fetch_schedule;
use anyhow::Result;

pub async fn populate_match_autocomplete(user_input: String) -> Result<Vec<(String, u64)>> {
    let schedule = fetch_schedule().await?;

    let mut matches = Vec::new();
    for game in &schedule.games {
        let title = format!(
            "{} vs. {}",
            game.home_team.place_name.default, game.away_team.place_name.default
        );
        if user_input.is_empty() || title.to_lowercase().contains(user_input.as_str()) {
            matches.push((title, game.id));
        }
    }
    Ok(matches)
}
