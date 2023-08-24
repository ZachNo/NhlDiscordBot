use serde::{Deserialize, Serialize};
use crate::data_model::nhl_team::NhlTeam;

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlScheduleGameTeam {
    pub score: u32,
    pub team: NhlTeam
}