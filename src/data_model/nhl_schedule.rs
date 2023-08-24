use serde::{Deserialize, Serialize};
use crate::data_model::nhl_schedule_date::NhlScheduleDate;

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlSchedule {
    pub dates: Vec<NhlScheduleDate>,
}

pub fn parse_schedule_data(data: &str) -> NhlSchedule {
    return serde_json::from_str(data).unwrap();
}