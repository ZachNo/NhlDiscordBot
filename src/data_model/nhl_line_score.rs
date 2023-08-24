use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlLineScore {
    pub current_period: u32,
    pub current_period_ordinal: String,
    pub current_period_time_remaining: String,
}