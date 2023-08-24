use serde::{Deserialize, Serialize};
use crate::data_model::nhl_schedule_game_team::NhlScheduleGameTeam;

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlScheduleGameTeams {
    pub away: NhlScheduleGameTeam,
    pub home: NhlScheduleGameTeam,
}