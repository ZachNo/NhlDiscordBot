use crate::nhl::model::common::TranslationString;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: u64,
    pub game_type: u8,
    pub venue: TranslationString,
    pub start_time_u_t_c: String,
    pub game_state: String,
    pub period: Option<u8>,
    pub away_team: Team,
    pub home_team: Team,
    pub clock: Option<Clock>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: TranslationString,
    pub score: Option<u8>,
    pub sog: Option<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub time_remaining: String,
}

pub fn parse_game_data(data: &str) -> Result<Game> {
    serde_json::from_str(data).context("parse game data")
}
