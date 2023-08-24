use serde::{Deserialize, Serialize};
use crate::data_model::{
    nhl_game_data::NhlGameData,
    nhl_live_data::NhlLiveData,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlLive {
    pub game_pk: u64,
    pub game_data: NhlGameData,
    pub live_data: NhlLiveData,
}

pub fn parse_live_data(data: &str) -> NhlLive {
    return serde_json::from_str(data).unwrap();
}