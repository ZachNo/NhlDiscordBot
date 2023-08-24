use serde::{Deserialize, Serialize};
use crate::data_model::nhl_game_status::NhlGameStatus;

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlGameData {
    pub status: NhlGameStatus,
}