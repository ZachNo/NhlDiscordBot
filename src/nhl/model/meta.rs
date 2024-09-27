use crate::nhl::model::common::TranslationString;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    //pub players: Vec<Player>,
    pub teams: Vec<Team>,
    //pub season_states: Vec<Season>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub name: TranslationString,
    pub tricode: String,
    pub team_id: u32,
}

pub fn parse_meta(data: &str) -> Result<Meta> {
    serde_json::from_str(data).context("parse meta data")
}
