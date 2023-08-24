use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlScheduleGameVenue {
    pub name: String,
}