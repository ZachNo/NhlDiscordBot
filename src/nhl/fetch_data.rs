use crate::error::DiscordError::NhlServerError;
use anyhow::{anyhow, Context, Result};
use cached::proc_macro::io_cached;
use cached::stores::{AsyncRedisCache, AsyncRedisCacheBuilder};
use chrono::{Local, TimeDelta};
use std::ops::Add;

use crate::nhl::model::{
    game::{parse_game_data, Game},
    schedule::{parse_schedule_data, Day},
};

const BASE_URL: &str = "https://api-web.nhle.com/v1";
const REDIS: &str = "redis://127.0.0.1:6379";

pub async fn fetch_today_schedule() -> Result<Day> {
    fetch_schedule(Local::now().format("%Y-%m-%d").to_string()).await
}

pub async fn fetch_tomorrow_schedule() -> Result<Day> {
    fetch_schedule(
        Local::now()
            .add(TimeDelta::days(1))
            .format("%Y-%m-%d")
            .to_string(),
    )
    .await
}

pub async fn fetch_yesterday_schedule() -> Result<Day> {
    fetch_schedule(
        Local::now()
            .add(TimeDelta::days(-1))
            .format("%Y-%m-%d")
            .to_string(),
    )
    .await
}

#[io_cached(
    map_error = r##"|e| anyhow!("redis: {:?}", e)"##,
    ty = "AsyncRedisCache<String, Day>",
    create = r##" {
        AsyncRedisCacheBuilder::new("schedule", 3600)
            .set_connection_string(REDIS)
            .set_refresh(true)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
async fn fetch_schedule(day: String) -> Result<Day> {
    let data = reqwest::get(format!("{BASE_URL}/schedule/{day}"))
        .await
        .map_err(|e| NhlServerError(e.to_string()))?
        .text()
        .await
        .map_err(|e| NhlServerError(e.to_string()))?;
    let schedule = parse_schedule_data(data.as_str()).with_context(|| format!("{day}"))?;
    Ok(schedule.game_week[0].clone())
}

#[io_cached(
    map_error = r##"|e| anyhow!("redis: {:?}", e)"##,
    ty = "AsyncRedisCache<u64, Game>",
    create = r##" {
        AsyncRedisCacheBuilder::new("match", 5)
            .set_connection_string(REDIS)
            .build()
            .await
            .expect("error building redis cache")
    } "##
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
