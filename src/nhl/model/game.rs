use crate::nhl::model::common::{Team, TranslationString};
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
    pub period_descriptor: Option<Period>,
    pub away_team: Team,
    pub home_team: Team,
    pub clock: Option<Clock>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub number: u8,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub time_remaining: String,
}

pub fn parse_game_data(data: &str) -> Result<Game> {
    serde_json::from_str(data).context("parse game data")
}
