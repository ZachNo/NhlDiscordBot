use serde::{Deserialize, Serialize};
use crate::data_model::{
    nhl_box_score::NhlBoxScore,
    nhl_line_score::NhlLineScore,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlLiveData {
    pub linescore: NhlLineScore,
    pub boxscore: NhlBoxScore,
}