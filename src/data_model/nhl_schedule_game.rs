use serde::{Deserialize, Serialize};
use crate::data_model::{
    nhl_game_status::NhlGameStatus,
    nhl_schedule_game_teams::NhlScheduleGameTeams,
    nhl_schedule_game_venue::NhlScheduleGameVenue,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlScheduleGame {
    pub game_pk: u64,
    game_type: String,
    season: String,
    pub game_date: String,
    pub status: NhlGameStatus,
    pub teams: NhlScheduleGameTeams,
    pub venue: NhlScheduleGameVenue,
}