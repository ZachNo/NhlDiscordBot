use cached::{
    proc_macro::cached,
    stores::{
        TimedCache,
        TimedSizedCache,
    },
};
use chrono::Local;
use crate::data_model::{
    nhl_content::{
        NhlContent,
        parse_content_data,
    },
    nhl_live::{
        NhlLive,
        parse_live_data
    },
    nhl_schedule::{
        NhlSchedule,
        parse_schedule_data,
    }
};

#[cached(
type = "TimedCache<(), NhlSchedule>",
create = "{ TimedCache::with_lifespan(3600) }"
)]
pub async fn fetch_schedule_from_web() -> NhlSchedule {
    let data = reqwest::get(format!(
        "https://statsapi.web.nhl.com/api/v1/schedule?date={}",
        Local::now().format("%F")
    )).await.unwrap().text().await.unwrap();
    parse_schedule_data(data.as_str())
}

#[cached(
type = "TimedCache<u64, NhlLive>",
create = "{ TimedCache::with_lifespan(5) }"
)]
pub async fn fetch_match_score(match_id: u64) -> NhlLive {
    let body: String = reqwest::get(format!("https://statsapi.web.nhl.com/api/v1/game/{}/feed/live", match_id.to_string()))
        .await.unwrap().text().await.unwrap();
    parse_live_data(body.as_str())
}

#[cached(
type = "TimedSizedCache<u64, NhlContent>",
create = "{ TimedSizedCache::with_size_and_lifespan(10, 10) }"
)]
pub async fn fetch_match_highlights(match_id: u64) -> NhlContent {
    let body: String = reqwest::get(format!("https://statsapi.web.nhl.com/api/v1/game/{}/content", match_id.to_string()))
        .await.unwrap().text().await.unwrap();
    parse_content_data(body.as_str())
}