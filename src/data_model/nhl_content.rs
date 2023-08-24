use serde::{Deserialize, Serialize};
use crate::data_model::nhl_highlights::NhlHighlights;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlContent {
    pub highlights: NhlHighlights,
}

pub fn parse_content_data(data: &str) -> NhlContent {
    return serde_json::from_str(data).unwrap();
}