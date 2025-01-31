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
    pub special_event: Option<TranslationString>,
}

impl Game {
    pub fn get_away_team_full_name(&self) -> String {
        if self.away_team.common_name.default.starts_with(self.away_team.place_name.default.as_str()) {
            self.away_team.common_name.default.clone()
        } else {
            format!("{} {}", self.away_team.place_name.default, self.away_team.common_name.default)
        }
    }

    pub fn get_home_team_full_name(&self) -> String {
        if self.home_team.common_name.default.starts_with(self.home_team.place_name.default.as_str()) {
            self.home_team.common_name.default.clone()
        } else {
            format!("{} {}", self.home_team.place_name.default, self.home_team.common_name.default)
        }
    }
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
