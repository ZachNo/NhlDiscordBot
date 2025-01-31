use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::nhl::model::game::Game;

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

pub fn parse_schedule_data(data: &str) -> Result<Schedule> {
    serde_json::from_str(data).context("parse schedule data")
}
