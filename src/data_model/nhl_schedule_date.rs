use serde::{Deserialize, Serialize};
use crate::data_model::nhl_schedule_game::NhlScheduleGame;

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlScheduleDate {
    pub date: String,
    pub games: Vec<NhlScheduleGame>,
}