use crate::nhl::model::common::{Team, TranslationString};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub game_week: Vec<Day>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Day {
    pub date: String,
    pub games: Vec<Game>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: u64,
    pub game_type: u8,
    pub venue: TranslationString,
    pub start_time_u_t_c: String,
    pub game_state: String,
    pub away_team: Team,
    pub home_team: Team,
    pub special_event: Option<TranslationString>,
}

pub fn parse_schedule_data(data: &str) -> Result<Schedule> {
    serde_json::from_str(data).context("parse schedule data")
}
