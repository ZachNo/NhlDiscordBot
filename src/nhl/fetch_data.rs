use crate::error::DiscordError::NhlServerError;
use anyhow::{anyhow, Result};
use cached::proc_macro::io_cached;
use cached::stores::{AsyncRedisCache, AsyncRedisCacheBuilder};
use chrono::{Local, TimeDelta};
use std::ops::Add;

use crate::nhl::model::meta::parse_meta;
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
    let schedule = parse_schedule_data(data.as_str());
    Ok(schedule?.game_week[0].clone())
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

#[io_cached(
    map_error = r##"|e| anyhow!("redis: {:?}", e)"##,
    ty = "AsyncRedisCache<u32, String>",
    create = r##" {
        AsyncRedisCacheBuilder::new("team", 86400)
            .set_connection_string(REDIS)
            .set_refresh(true)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn fetch_team_name(team_id: u32) -> Result<String> {
    let body: String = reqwest::get(format!("{BASE_URL}/meta?teams={team_id}"))
        .await
        .map_err(|e| NhlServerError(e.to_string()))?
        .text()
        .await
        .map_err(|e| NhlServerError(e.to_string()))?;
    parse_meta(body.as_str()).map(|t| t.teams[0].name.default.clone())
}
