use anyhow::Result;
use cached::{proc_macro::cached, stores::TimedCache};
use chrono::Local;

use crate::nhl::model::{
    game::{parse_game_data, Game},
    schedule::{parse_schedule_data, Day},
};

const BASE_URL: &str = "https://api-web.nhle.com/v1";

#[cached(
    type = "TimedCache<(), Day>",
    create = "{ TimedCache::with_lifespan(3600) }",
    result = true
)]
pub async fn fetch_schedule() -> Result<Day> {
    let data = reqwest::get(format!(
        "{}/schedule/{}",
        BASE_URL,
        Local::now().format("%Y-%m-%d")
    ))
    .await?
    .text()
    .await?;
    let schedule = parse_schedule_data(data.as_str());
    Ok(schedule?.game_week[0].clone())
}

#[cached(
    type = "TimedCache<u64, Game>",
    create = "{ TimedCache::with_lifespan(5) }",
    result = true
)]
pub async fn fetch_match_score(match_id: u64) -> Result<Game> {
    let body: String = reqwest::get(format!(
        "{}/gamecenter/{}/boxscore",
        BASE_URL,
        match_id.to_string()
    ))
    .await?
    .text()
    .await?;
    parse_game_data(body.as_str())
}
