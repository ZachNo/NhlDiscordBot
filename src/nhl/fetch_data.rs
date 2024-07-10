use crate::error::DiscordError::NhlServerError;
use anyhow::Result;
use cached::{proc_macro::cached, stores::TimedCache};
use chrono::{Local, TimeDelta};
use std::ops::Add;

use crate::nhl::model::{
    game::{parse_game_data, Game},
    schedule::{parse_schedule_data, Day},
};

const BASE_URL: &str = "https://api-web.nhle.com/v1";

#[cached(
    ty = "TimedCache<(), Day>",
    create = "{ TimedCache::with_lifespan(3600) }",
    result = true
)]
pub async fn fetch_today_schedule() -> Result<Day> {
    fetch_schedule(Local::now().format("%Y-%m-%d").to_string()).await
}

#[cached(
    ty = "TimedCache<(), Day>",
    create = "{ TimedCache::with_lifespan(3600) }",
    result = true
)]
pub async fn fetch_tomorrow_schedule() -> Result<Day> {
    fetch_schedule(
        Local::now()
            .add(TimeDelta::days(1))
            .format("%Y-%m-%d")
            .to_string(),
    )
    .await
}

#[cached(
    ty = "TimedCache<(), Day>",
    create = "{ TimedCache::with_lifespan(3600) }",
    result = true
)]
pub async fn fetch_yesterday_schedule() -> Result<Day> {
    fetch_schedule(
        Local::now()
            .add(TimeDelta::days(-1))
            .format("%Y-%m-%d")
            .to_string(),
    )
    .await
}

async fn fetch_schedule(day: String) -> Result<Day> {
    let data = reqwest::get(format!("{BASE_URL}/schedule/{day}"))
        .await
        .map_err(|e| NhlServerError(e.to_string()))?
        .text()
        .await
        .map_err(|e| NhlServerError(e.to_string()))?;
    let schedule = parse_schedule_data(data.as_str());
    Ok(schedule?.game_week[0].clone())
}

#[cached(
    ty = "TimedCache<u64, Game>",
    create = "{ TimedCache::with_lifespan(5) }",
    result = true
)]
pub async fn fetch_match_score(match_id: u64) -> Result<Game> {
    let body: String = reqwest::get(format!("{BASE_URL}/gamecenter/{match_id}/boxscore"))
        .await
        .map_err(|e| NhlServerError(e.to_string()))?
        .text()
        .await
        .map_err(|e| NhlServerError(e.to_string()))?;
    parse_game_data(body.as_str())
}
